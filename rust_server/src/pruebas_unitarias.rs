#[cfg(test)]
mod tests {
    use crate::estado::{EstadoServidor};
    use crate::protocolo::*;
    use crate::estado::Salas;
    use crate::{usuarios, salas, manejador}; 
    use tokio::sync::{mpsc, Mutex};
    use std::sync::Arc;

    // Para crear el estado de prueba
    async fn setup_entorno() -> (
        Arc<Mutex<EstadoServidor>>,
        mpsc::UnboundedReceiver<MensajesDeSalida>,
        mpsc::UnboundedSender<MensajesDeSalida>,
    ) {
        let estado = Arc::new(Mutex::new(EstadoServidor::nuevo()));
        let (tx_cliente, rx_cliente) = mpsc::unbounded_channel();
        (estado, rx_cliente, tx_cliente)
    }

    async fn setup_sala_un_usuario(
        estado: &Arc<Mutex<EstadoServidor>>,
        roomname: &str,
        usuario: &str,
        tx_cliente: mpsc::UnboundedSender<MensajesDeSalida>,
    ) {
        let mut memoria = estado.lock().await;
        memoria.usuarios.insert(usuario.to_string(), (tx_cliente, EstadoUsuario::ACTIVE));

        let mut miembros = std::collections::HashSet::new();
        miembros.insert(usuario.to_string());

        memoria.salas.insert(roomname.to_string(), Salas {
            dueno_sala: usuario.to_string(),
            miembros,
            invitados: std::collections::HashSet::new(),
        });
    }

    async fn setup_sala_dos_usuarios(
        estado: &Arc<Mutex<EstadoServidor>>,
        roomname: &str,
        dueno: &str,
        tx_dueno: mpsc::UnboundedSender<MensajesDeSalida>,
        miembro2: &str,
        tx_miembro2: mpsc::UnboundedSender<MensajesDeSalida>,
    ) {
        let mut memoria = estado.lock().await;
        
        memoria.usuarios.insert(dueno.to_string(), (tx_dueno, EstadoUsuario::ACTIVE));
        memoria.usuarios.insert(miembro2.to_string(), (tx_miembro2, EstadoUsuario::ACTIVE));

        let mut miembros = std::collections::HashSet::new();
        miembros.insert(dueno.to_string());
        miembros.insert(miembro2.to_string());

        memoria.salas.insert(roomname.to_string(), Salas {
            dueno_sala: dueno.to_string(),
            miembros,
            invitados: std::collections::HashSet::new(),
        });
    }

    async fn setup_sala_con_invitado(
        estado: &Arc<Mutex<EstadoServidor>>,
        roomname: &str,
        dueno: &str,
        tx_dueno: mpsc::UnboundedSender<MensajesDeSalida>,
        invitado: &str,
        tx_invitado: mpsc::UnboundedSender<MensajesDeSalida>,
    ) {
        let mut memoria = estado.lock().await;
        
        memoria.usuarios.insert(dueno.to_string(), (tx_dueno, EstadoUsuario::ACTIVE));
        memoria.usuarios.insert(invitado.to_string(), (tx_invitado, EstadoUsuario::ACTIVE));

        let mut miembros = std::collections::HashSet::new();
        miembros.insert(dueno.to_string());

        let mut invitados = std::collections::HashSet::new();
        invitados.insert(invitado.to_string());

        memoria.salas.insert(roomname.to_string(), Salas {
            dueno_sala: dueno.to_string(),
            miembros,
            invitados,
        });
    }

    //Inicio de los test, lo de arriba son fn auxiliares porque sí, viva la programación estructurada
    #[tokio::test] 
    async fn test_no_existe_usuario(){
        let (estado, mut _rx, tx_cliente) = setup_entorno().await;
        let mut nombre_actual = Some("Aly".to_string());
        
        let linea_txt = r#"{"type": "TEXT", "username": "Bob", "text": "Hola Bob"}"#.to_string();
        
        let respuesta = manejador::procesar_json(
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
            _ => panic!("Se espera un NO_SUCH_USER como respuesta")
        }
    }

    #[tokio::test]
    async fn test_texto_bien(){
        let (estado, mut _rx, tx_aly) = setup_entorno().await;
        
        // Setup Bob
        let (tx_bob, mut rx_bob) = mpsc::unbounded_channel();
        {
            let mut memoria = estado.lock().await;
            memoria.usuarios.insert("Bob".to_string(), (tx_bob, EstadoUsuario::ACTIVE));
        }

        let mut nombre_aly = Some("Aly".to_string());
        let msj_entrada = r#"{"type": "TEXT", "username": "Bob", "text": "Hola, Bob"}"#.to_string();

        let respuesta = manejador::procesar_json(
            &msj_entrada,
            estado.clone(),
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
        let (estado, mut _rx, tx_cliente) = setup_entorno().await;
        let mut nombre_actual: Option<String> = None;
        let nuevo_usuario = "Aly".to_string();

        let respuesta = usuarios::procesar_identify(&estado, &mut nombre_actual, nuevo_usuario, tx_cliente).await;

        assert_eq!(nombre_actual, Some("Aly".to_string()), "El nombre actual debió actualizarse");
        
        let memoria = estado.lock().await;
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
        let emisor = "Aly".to_string();
        let room_name = "Sala_Prueba".to_string();

        let respuesta = salas::procesar_new_room(&estado, room_name.clone(), emisor).await;

        let memoria = estado.lock().await;
        assert!(memoria.salas.contains_key("Sala_Prueba"), "La sala debió crearse en la memoria");

        let sala_creada = memoria.salas.get("Sala_Prueba").unwrap();
        assert_eq!(sala_creada.dueno_sala, "Aly", "El dueño debe de ser el emisor");
        assert!(sala_creada.miembros.contains("Aly"), "El emisor debe de estar en los miembros");
        assert!(sala_creada.invitados.is_empty(), "La sala de invitados debe estar vacia");

        match respuesta {
            Some(MensajesDeSalida::RESPONSE { resultado, ..}) => {
                assert_eq!(resultado, ResultadoOperacion::SUCCESS, "La operación debió ser exitosa");
            },
            _ => panic!("Respuesta incorrecta por parte del NEW_ROOM"),
        }
    }

    #[tokio::test]
    async fn join_room_exitoso() {
        let (estado, mut _rx, tx_cliente) = setup_entorno().await;
        let emisor = "Bob".to_string();
        let roomname = "Sala_Prueba".to_string();
        let (tx_aly, _rx_aly) = tokio::sync::mpsc::unbounded_channel();

        setup_sala_con_invitado(&estado, &roomname, "Aly", tx_aly, &emisor, tx_cliente).await;

        let respuesta = salas::procesar_join_room(&estado, roomname, emisor).await;

        let memoria = estado.lock().await;
        let sala_actualizada = memoria.salas.get("Sala_Prueba").unwrap();

        assert!(sala_actualizada.miembros.contains("Bob"), "Bob, debió ser agregado correctamente");
        assert!(!sala_actualizada.invitados.contains("Bob"), "Bob ya no debe de aparece en invitados");

        match respuesta {
            Some(MensajesDeSalida::RESPONSE { resultado, .. }) => {
                assert_eq!(resultado, ResultadoOperacion::SUCCESS, "La operacion debió devolver SUCCESS");
            },
            _ => panic!("Respuesta equivocada para un SUCCESS"),
        }
    }

    #[tokio::test]
    async fn joinroom_no_invitado() {
        let (estado, _, _tx_cliente) = setup_entorno().await;
        let (tx_aly, _rx_aly) = tokio::sync::mpsc::unbounded_channel();
        let emisor = "Charlie".to_string();
        let roomname = "Sala_Prueba".to_string();

        setup_sala_un_usuario(&estado, &roomname, "Aly", tx_aly).await;

        // Llamamos al módulo (Charlie NO fue invitado)
        let respuesta = salas::procesar_join_room(&estado, roomname, emisor).await;

        let memoria = estado.lock().await;
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

        setup_sala_dos_usuarios(
            &estado, 
            &roomname,
            &emisor,
            tx_cliente.clone(),
            &otro_usuario,
            tx_cliente.clone()).await;

        // Llamamos al módulo
        let respuesta = salas::procesar_room_users(&estado, roomname, emisor).await;

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

        setup_sala_un_usuario(&estado, &roomname, "Aly", tx_cliente).await;

        
        let respuesta = salas::procesar_room_users(&estado, roomname, emisor).await;

        match respuesta {
            Some(MensajesDeSalida::RESPONSE {resultado, .. }) => {
                assert_eq!(resultado, ResultadoOperacion::NOT_JOINED, "Debe rechazar a Charlie");
            },
            _ => panic!("El server no detectó al metiche de Charlie")
        }
    }

    #[tokio::test]
    async fn test_text_room_exitoso() {
        let (estado, _, tx_emisor) = setup_entorno().await;
        let emisor = "Aly".to_string();
        let roomname = "Sala_Prueba".to_string();
        let txt_enviado = "Muy buenas".to_string();

        let (tx_bob, mut rx_bob) = mpsc::unbounded_channel();

        setup_sala_dos_usuarios(&estado, &roomname, "Aly", tx_emisor, "Bob", tx_bob).await;

        let respuesta = salas::procesar_room_text(&estado, roomname, txt_enviado, emisor).await;

        // El server NO responde al emisor
        assert!(respuesta.is_none(), "El server no debe de responder al emisor");

        // Verificamos el canal del receptor (Bob)
        let msj_bob = rx_bob.try_recv().expect("Bob debe de recibir el msj de la sala");
        match msj_bob {
            MensajesDeSalida::ROOM_TEXT_FROM { roomname: r, username: u, text: t } => {
                assert_eq!(r, "Sala_Prueba");
                assert_eq!(u, "Aly", "El msj debe de indicar que Aly lo envió");
                assert_eq!(t, "Muy buenas");
            },
            _ => panic!("Bob recibió un mensaje distinto"),
        }
    }

    #[tokio::test]
    async fn test_text_room_no_exitoso(){
        let (estado, _, tx_cliente) = setup_entorno().await;
        let roomname = "Sala_Prueba".to_string();
        let emisor = "Charlie".to_string();
        let txt = "Mensaje pirata".to_string();

        setup_sala_un_usuario(&estado, &roomname, "Aly", tx_cliente).await;

        let respuesta = salas::procesar_room_text(&estado, roomname, txt, emisor).await;

        match respuesta {
            Some(MensajesDeSalida::RESPONSE { resultado, .. }) => {
                assert_eq!(resultado, ResultadoOperacion::NOT_JOINED, "Debe rechazar a Charlie");
            },
            _ => panic!("El server no bloqueó el envío de Charlie"),
        }
    }
}