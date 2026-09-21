use crate::estado::{EstadoServidor};
    use super::*;
    use tokio::sync::{mpsc, Mutex};
    use std::sync::Arc;

    //Para crear el estado de prueba y no escribir todo varias veces lol 
    async fn setup_entorno() -> (
        Arc<Mutex<EstadoServidor>>,
        mpsc::UnboundedReceiver<MensajesDeSalida>,
        mpsc::UnboundedSender<MensajesDeSalida>,
    ) {
        let estado = Arc::new(Mutex::new(EstadoServidor::nuevo()));
        let (tx_cliente, rx_cliente) = mpsc::unbounded_channel();
        (estado, rx_cliente, tx_cliente)
    }
    //Si el destinatario no existe
    #[tokio::test] 
    async fn test_no_existe_usuario(){

        let (estado, mut _rx, tx_cliente) = setup_entorno().await;
        let mut nombre_actual = Some("Aly".to_string());
        
        let linea_txt = r#"{"type": "TEXT", "username": "Bob", "text": "Hola Bob"}"#.to_string();

        
        let respuesta = procesar_json(
            &linea_txt,
            estado.clone(),
            tx_cliente,
            &mut nombre_actual,
        ).await;

        match respuesta {
            Some(MensajesDeSalida::RESPONSE {operation,resultado,extra }) => {
                assert_eq!(operation, Operacion::TEXT); 
                assert_eq!(resultado, ResultadoOperacion::NO_SUCH_USER);
                assert_eq!(extra, Some("Bob".to_string()));
            }
            _ => {
                panic!("Se espera un NO_SUCH_USER como respuesta")
            }
        }
    }

    #[tokio::test]
    async fn test_texto_bien(){
        let estado_mock = Arc::new(Mutex::new(EstadoServidor::nuevo()));

        //Destino
        let (tx_bob,  mut rx_bob) = mpsc::unbounded_channel();
        estado_mock.lock().await.usuarios.insert(
            "Bob".to_string(),
            (tx_bob, EstadoUsuario::ACTIVE)
        );


        //Quien manda el msj
        let (tx_aly, _rx_aly) = mpsc::unbounded_channel();
        let mut nombre_aly = Some("Aly".to_string());
        let msj_entrada = r#"{"type": "TEXT", "username": "Bob", "text": "Hola, Bob"}"#.to_string();


        let respuesta = procesar_json(
            &msj_entrada,
            estado_mock,
            tx_aly,
            &mut nombre_aly,
        ).await;

        assert!(respuesta.is_none());

        let msj_para_bob = rx_bob.recv().await.unwrap();
        match msj_para_bob {
            MensajesDeSalida::TEXT_FROM {username, text} => {
                assert_eq!(username, "Aly");
                assert_eq!(text, "Hola, Bob");
            }
            _ => panic!("Bob debió recibir un TEXT_FROM"),
        }


    }

    #[tokio::test]
    async fn identify_exitoso() {
        let (estado,mut _rx, tx_cliente) = setup_entorno()  .await;
        let mut nombre_actual: Option<String> = None;
        let nuevo_usuario = "Aly".to_string();

        let mut memoria = estado.lock().await;

        let respuesta = if memoria.usuarios.contains_key(&nuevo_usuario) {
            Some(MensajesDeSalida::RESPONSE {
                operation: Operacion::IDENTIFY,
                resultado: ResultadoOperacion::USER_ALREADY_EXISTS,
                extra: Some(nuevo_usuario.clone()),
            })
        }else {
            memoria.usuarios.insert(
                nuevo_usuario.clone(),
                (tx_cliente.clone(), EstadoUsuario::ACTIVE),
            );
            nombre_actual = Some(nuevo_usuario.clone());

            Some(MensajesDeSalida::RESPONSE {
                operation: Operacion::IDENTIFY,
                resultado: ResultadoOperacion::SUCCESS,
                extra: Some(nuevo_usuario),
            })
        };

        assert_eq!(nombre_actual, Some("Aly".to_string()), "El nombre actual debió actualizarse");
        assert!(memoria.usuarios.contains_key("Aly"), "El usuario debió guardarse en el HashMap");

        match respuesta {
            Some(MensajesDeSalida::RESPONSE { resultado, ..}) => {
                assert_eq!(resultado, ResultadoOperacion::SUCCESS, "La operación debió ser exitosa");
            },
            _ => panic!("Respuesta incorrecta por parte del IDENTIFY"),
        }
    }

    #[tokio::test]
    async fn new_room_exitoso(){
        let (estado, _ , _) = setup_entorno().await;
        let nombre_actual = Some("Aly".to_string());
        let room_name = "Sala_Prueba".to_string();

        let emisor = nombre_actual.unwrap();
        let mut memoria = estado.lock().await;

        let _respuesta = if memoria.salas.contains_key(&room_name) {
            None
        }else {
            let mut miembros_iniciales = std::collections::HashSet::new();
            miembros_iniciales.insert(emisor.clone());

            let sala_nueva = Salas {
                dueno_sala: emisor.clone(),
                miembros: miembros_iniciales,
                invitados: std::collections::HashSet::new(),
            };

            memoria.salas.insert(
                room_name.clone(),
                sala_nueva,
            );

            Some(MensajesDeSalida::RESPONSE {
                operation: Operacion::NEW_ROOM,
                resultado: ResultadoOperacion::SUCCESS,
                extra: Some(room_name.clone()),
            })
        };

        //Verifiaciones
        assert!(memoria.salas.contains_key("Sala_Prueba"), "La sala debió crearse en la memoria");

        let sala_creada = memoria.salas.get("Sala_Prueba").unwrap();
        assert_eq!(sala_creada.dueno_sala, ("Aly"), "El dueño debe de ser el emisor");
        assert!(sala_creada.miembros.contains("Aly"), "El emisor debe de estar en los miembros");
        assert!(sala_creada.invitados.is_empty(), "La sala de invitados debe de esstar vacia");
    }

    #[tokio::test]
    async fn join_room_exitoso() {
        let (estado, mut _rx, tx_cliente) = setup_entorno().await;
        let nombre_actual = Some("Bob".to_string());
        let roomname = "Sala_Prueba".to_string();
        let emisor = nombre_actual.unwrap();
        let mut memoria = estado.lock().await;


        { //Creamos la sala y poemos al buen BOB en la sala de invitados
            memoria.usuarios.insert(emisor.clone(), (tx_cliente, EstadoUsuario::ACTIVE));

            let mut miembros = std::collections::HashSet::new();
            miembros.insert("Aly".to_string()); // Dueño de la sala

            let mut invitados = std::collections::HashSet::new();
            invitados.insert(emisor.clone()); // Bob está invitado

            memoria.salas.insert(roomname.clone(), Salas {
                dueno_sala: "Aly".to_string(),
                miembros,
                invitados,
            });
        }

        let respuesta_generada = {
            let EstadoServidor { ref usuarios, ref mut salas } = *memoria;

            let sala = salas.get_mut(&roomname).unwrap();

            if !sala.invitados.contains(&emisor){
                Some(MensajesDeSalida::RESPONSE {
                    operation: Operacion::JOIN_ROOM,
                    resultado: ResultadoOperacion::NOT_INVITED,
                    extra: Some(roomname.clone()),
                })
            }else {
                sala.invitados.remove(&emisor);
                sala.miembros.insert(emisor.clone());

                let msj_emisor = MensajesDeSalida::JOINED_ROOM {
                    roomname: roomname.clone(),
                    username: emisor.clone(),
                };

                for miembro in &sala.miembros{
                    if let Some((tx_destino, _)) = usuarios.get(miembro){
                        let _ = tx_destino.send(msj_emisor.clone());
                    }
                }

                Some(MensajesDeSalida::RESPONSE {
                    operation: Operacion::JOIN_ROOM,
                    resultado: ResultadoOperacion::SUCCESS,
                    extra: Some(roomname.clone()),
                })

            }
        };


        let sala_actualizada = memoria.salas.get("Sala_Prueba").unwrap();

        assert!(sala_actualizada.miembros.contains("Bob"), "Bob, debió ser agregado correctamente");
        assert!(!sala_actualizada.invitados.contains("Bob"), "Bob ya no debe de aparece en invitados");

        match respuesta_generada {
            Some(MensajesDeSalida::RESPONSE { resultado, .. }) => {
                assert_eq!(resultado, ResultadoOperacion::SUCCESS, "La operacion debió devolver SUCCESS");
            },
            _ => panic!("Respuesta equivocada para un SUCCESS"),
        }

    }

    #[tokio::test]
    async fn joinroom_no_invitado() {
        let (estado, _, tx_cliente) = setup_entorno().await;
        let emisor = "Charlie".to_string();
        let roomname = "Sala_Prueba".to_string();

        {
            let mut memoria = estado.lock().await;
            memoria.usuarios.insert(emisor.clone(), (tx_cliente, EstadoUsuario::ACTIVE));
            
            let mut miembros = std::collections::HashSet::new();
            miembros.insert("Aly".to_string());

            memoria.salas.insert(roomname.clone(), Salas {
                dueno_sala: "Aly".to_string(),
                miembros,
                invitados: std::collections::HashSet::new(),
            });
        }

        let mut memoria = estado.lock().await;
        let EstadoServidor {ref mut salas, .. } = *memoria;

        let respuesta = if let Some(salas) = salas.get_mut(&roomname) {
            if !salas.invitados.contains(&emisor) {
                Some(MensajesDeSalida::RESPONSE {
                    operation: Operacion::JOIN_ROOM,
                    resultado: ResultadoOperacion::NOT_INVITED,
                    extra: Some(roomname.clone()),
                })
            }else {
                None
            }
        }else {
            None
        };

        let sala_actual = memoria.salas.get("Sala_Prueba").unwrap();
        assert!(!sala_actual.miembros.contains("Charlie"), "Charlie NO debió entrar a los miembros");

        match respuesta {
            Some(MensajesDeSalida::RESPONSE {resultado, .. }) => {
                assert_eq!(resultado, ResultadoOperacion::NOT_INVITED, "El server debe de rechazar dicha peticion");
            }
            _ => panic!("El server debe de detectar que el usuario no estaba invitado"),
        }
    }

    #[tokio::test]
    async fn test_room_users(){
        let (estado, _, tx_cliente) = setup_entorno().await;
        let emisor = "Aly".to_string();
        let otro_usuario = "Bob".to_string();
        let roomname = "Sala_Prueba".to_string();

        {
        
            let mut memoria = estado.lock().await;
            memoria.usuarios.insert(emisor.clone(), (tx_cliente.clone(), EstadoUsuario::ACTIVE));
            memoria.usuarios.insert(otro_usuario.clone(), (tx_cliente, EstadoUsuario::ACTIVE));

            let mut miembros = std::collections::HashSet::new();
            miembros.insert(emisor.clone());
            miembros.insert(otro_usuario.clone());

            memoria.salas.insert(roomname.clone(), Salas {
                dueno_sala: emisor.clone(),
                miembros,
                invitados: std::collections::HashSet::new(),
            });

        }

        let memoria = estado.lock().await;

        let respuesta = {
            let EstadoServidor { ref usuarios, ref salas } = *memoria;

            let sala = salas.get(&roomname).unwrap();

            if !sala.miembros.contains(&emisor){
                Some(MensajesDeSalida::RESPONSE {
                    operation: Operacion::ROOM_USERS,
                    resultado: ResultadoOperacion::NOT_JOINED,
                    extra: Some(roomname.clone()),
                })
            }else {
                let mut diccionario_usuarios = std::collections::HashMap::new();
                for miembro in &sala.miembros {
                    if let Some((_tx, estado_usuario)) = usuarios.get(miembro){
                        diccionario_usuarios.insert(miembro.clone(), format!("{:?}", estado_usuario ));
                    }
                }

                Some(MensajesDeSalida::ROOM_USER_LIST {
                    roomname: roomname.clone(),
                    users: diccionario_usuarios,
                })
            }
        };

        match respuesta {

            Some(MensajesDeSalida::ROOM_USER_LIST { roomname: resp_room, users }) => {
                assert_eq!(resp_room, "Sala_Prueba", "El nombre de la sala debe coincidir");
                assert_eq!(users.len(), 2, "Deben aparecer exactamente 2 usuarios en la lista");
                assert_eq!(users.get("Aly").unwrap(), "ACTIVE", "Aly debe tener estado ACTIVE");
                assert_eq!(users.get("Bob").unwrap(), "ACTIVE", "Bob debe tener estado ACTIVE");
            },
            _ => panic!("La operación debió devolver la variante ROOM_USER_LIST con el diccionario"),
        }
    }

    #[tokio::test]
    async fn test_room_users_rechazo () {
        let (estado, _, tx_cliente) = setup_entorno().await;
        let emisor = "Charlie".to_string();
        let roomname = "Sala_Prueba".to_string();

        //Una sala solita con 1 miembro
        {
            let mut memoria = estado.lock().await;
            memoria.usuarios.insert(emisor.clone(), (tx_cliente, EstadoUsuario::ACTIVE));

            let mut miembros = std::collections::HashSet::new();
            miembros.insert("Aly".to_string()); 

            memoria.salas.insert(roomname.clone(), Salas {
                dueno_sala: "Aly".to_string(),
                miembros,
                invitados: std::collections::HashSet::new(),
            });
        }

        let memoria = estado.lock().await;
        let respuesta = {
            let EstadoServidor { ref salas, .. } = *memoria;

            if let Some(sala) = salas.get(&roomname) {
                if !sala.miembros.contains(&emisor){
                    Some(MensajesDeSalida::RESPONSE {
                        operation: Operacion::ROOM_USERS,
                        resultado: ResultadoOperacion::NOT_JOINED,
                        extra: Some(roomname.clone()),
                    })
                }else {
                    None
                }
            }else {
                None
            }
        };

        match respuesta {
            Some(MensajesDeSalida::RESPONSE {resultado, .. }) => {
                assert_eq!(
                    resultado,
                    ResultadoOperacion::NOT_JOINED,
                    "El server debe de bloquear a Charlie con NOT_JOINED por metiche"
                );
            },
            _ => panic!("El server no detectó al metiche de Cahrlie")
        }
    }