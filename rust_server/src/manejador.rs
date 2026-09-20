use std::collections::HashMap;

//Ahorita solo serán los mensajes de entrada, a un no muestra nda de msj de salida
use crate::protocolo::{MensajesDeEntrada, MensajesDeSalida, Operacion, ResultadoOperacion, EstadoUsuario};
use crate::estado::{EstadoCompartido, Salas, Transmisor, EstadoServidor};
use log::{info, error};

pub async fn procesar_json(
    linea_txt : &str,
    estado: EstadoCompartido,
    tx_cliente: Transmisor,
    nombre_actual: &mut Option<String>,
) -> Option<MensajesDeSalida> {

    //Inicio de los casos para deserializar el JSON dsjf
    match serde_json::from_str::<MensajesDeEntrada>(linea_txt){
        Ok(comando) => {
            info!("Coso del comando bien recibido: {:?}", comando);

            match comando{
                //Inicia a checar que parte del comando del JSON
                MensajesDeEntrada::IDENTIFY{username: nuevo_usuario} => {
                    info!("Nombre del cliente: {}", nuevo_usuario);

                    //Modificador de la memoria
                    let mut memoria =  estado.lock().await;

                    //Ver si el nombre no existe
                    if memoria.usuarios.contains_key(&nuevo_usuario){
                        return Some(MensajesDeSalida::RESPONSE {
                            operation: Operacion::IDENTIFY,
                            resultado: ResultadoOperacion::USER_ALREADY_EXISTS,
                            extra: Some(nuevo_usuario),
                        });
                    }

                    let msj_notificacion = MensajesDeSalida::NEW_USER {
                        username: nuevo_usuario.clone(), 
                    };

                    for(_nombre, (tx_destino, _estado)) in memoria.usuarios.iter(){
                        let _ = tx_destino.send(msj_notificacion.clone());
                    }

                    //Si no existe 
                    memoria.usuarios.insert(
                        nuevo_usuario.clone(),
                        (tx_cliente.clone(),EstadoUsuario::ACTIVE )
                    );

                    *nombre_actual = Some(nuevo_usuario.clone());
                    

                    //Retorno del mensaje
                    Some(MensajesDeSalida::RESPONSE {
                        operation: Operacion::IDENTIFY,
                        resultado: ResultadoOperacion::SUCCESS,
                        extra: Some(nuevo_usuario),
                    })
                }

                MensajesDeEntrada::STATUS { status: nuevo_estado } => {
                    info!("Cambio de estado del usuario a: {:?}", nuevo_estado);
                    //Aquí se debe de actualizar el estado en memoria y pasarlo a todos los usuarios

                    //Memoria para buscar al usuario
                    let mut memoria = estado.lock().await;

                    //Se busca que esté identificó al usuario
                    let emisor = match nombre_actual {
                        //Si el nombre existe
                        Some(nombre) => nombre.clone(),
                        //Si no existe el nombre
                        None => {
                            return  Some(MensajesDeSalida::RESPONSE {
                                operation: Operacion::INVALID,
                                resultado: ResultadoOperacion::NOT_IDENTIFIED,
                                extra: None,
                            });
                        }
                    };

                    if let Some((_tx, estado_actual)) = memoria.usuarios.get_mut(&emisor) {
                        if *estado_actual != nuevo_estado {
                            *estado_actual = nuevo_estado.clone();

                            let msj_usuarios = MensajesDeSalida::NEW_STATUS {
                                username: emisor.clone(),
                                status: nuevo_estado,
                            };

                            for (nombre,(tx_destino, _)) in memoria.usuarios.iter(){
                                if *nombre != emisor {
                                    let _ = tx_destino.send(msj_usuarios.clone()); 
                                }
                            }
                        }
                    }

                    None
                }

                MensajesDeEntrada::USERS => {
                    info!("Se solicita la lista de usuarios");
                    //LEER EL FKING HASH MAP Y DEVOLVER USER_LIST  
                    let memoria = estado.lock().await;

                    let _emisor = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(mensaje_error) => return Some(mensaje_error),
                    };

                    let mut lista_usuarios :HashMap<String, String> = HashMap::new();

                    for(nombre, (_tx, estado)) in memoria.usuarios.iter(){

                        let estado_str = format!("{:?}", estado);
                        lista_usuarios.insert(nombre.clone(), estado_str);
                    }

                    
                    Some(MensajesDeSalida::USER_LIST {
                        users: lista_usuarios,
                    })

                }

                MensajesDeEntrada::TEXT { username: destinatario, text } => {
                    info!("Mensaje para {}: {}", destinatario, text );
                    //Buscar el socket del destinatario y devovler TEXT_FROM
                    
                    let emisor = match nombre_actual {
                        Some(nombre) => nombre.clone(),
                        None => {
                            error!("Un usuario no identificado quiere mandar un msj");
                            return None;
                        }
                    };

                    //Lector de la memoria 
                    let memoria = estado.lock().await;

                    //Buscamos al usuario que este en el HashMap 
                    match memoria.usuarios.get(&destinatario) {
                        Some((tx_destino, _estado)) => {

                            let msj = MensajesDeSalida::TEXT_FROM {
                                username: emisor,
                                text: text.clone(), 
                            };

                            let _ = tx_destino.send(msj);

                            None
                        }
                        None => {
                            //Si el usuario no existe
                            Some(MensajesDeSalida::RESPONSE {
                                operation: Operacion::TEXT,
                                resultado: ResultadoOperacion::NO_SUCH_USER,
                                extra: Some(destinatario.clone()),
                            })
                        }
                    }

                }

                MensajesDeEntrada::PUBLIC_TEXT { text } => {
                    info!("Mensaje general: {}", text);
                    // Enviar el texto a todos los usuarios que esten en la red
                    let emisor  = match nombre_actual {
                        Some(nombre) => nombre.clone(),
                        None => {
                            error!("Un usuario no identificado quiere mandar un msj");
                            return None;
                        }
                    };

                    //Lector de la memomria
                    let memoria = estado.lock().await;

                    //Manda el msj a todos menos al emisor, usa un for que aquí es un iterador
                    for (destinatario, (tx_destino, _estado)) in memoria.usuarios.iter(){
                        if destinatario != &emisor{
                            let msj = MensajesDeSalida::PUBLIC_TEXT_FROM {
                                username: emisor.clone(),
                                text: text.clone(),
                            };

                            //Se manda el msj al otro usuario
                            let _ = tx_destino.send(msj);
                        }
                    }

                    None
                }

                MensajesDeEntrada::NEW_ROOM { roomname } => {
                    info!("Se crea una nueva sala, llamada: {}", roomname);
                    //Tengo que meter la lógica para crear las salas

                    let mut memoria = estado.lock().await;

                    let emisor = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(e) => return Some(e),
                    };

                    //Si ya existe la sala 
                    if memoria.salas.contains_key(&roomname){
                        return Some(MensajesDeSalida::RESPONSE {
                            operation: Operacion::NEW_ROOM,
                            resultado: ResultadoOperacion::ROOM_ALREADY_EXISTS,
                            extra: Some(roomname),
                        });
                    }

                    let mut miembros_iniciales  = std::collections::HashSet::new();
                    miembros_iniciales.insert(emisor.clone()); //Solo está el que creo la sala

                    let nueva_sala = Salas {
                        dueno_sala: emisor.clone(),
                        miembros: miembros_iniciales,
                        invitados: std::collections::HashSet::new(),
                    };

                    memoria.salas.insert(roomname.clone(), nueva_sala);

                    Some(MensajesDeSalida::RESPONSE {
                        operation: Operacion::NEW_ROOM,
                        resultado: ResultadoOperacion::SUCCESS,
                        extra: Some(roomname),
                    });

                    None
                }

                MensajesDeEntrada::INVITE { roomname, usernames } => {
                    info!("Invitar de {} a {:?}", roomname, usernames);
                    //Algo tengo que hacer para las invitaciones 

                    let emisor = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(e) => return Some(e),
                    };

                    let mut memoria = estado.lock().await;

                    let EstadoServidor { ref mut salas, ref usuarios } = *memoria;

                    //Validar que exista la sala
                    let sala = match salas.get_mut(&roomname) {
                        Some(s) => s,
                        None => {
                            return Some(MensajesDeSalida::RESPONSE {
                                operation: Operacion::INVITE,
                                resultado: ResultadoOperacion::NO_SUCH_ROOM,
                                extra: Some(roomname),
                            });
                        }
                    };

                    //Si el usuario no esta en la sala
                    if !sala.miembros.contains(&emisor){
                        return  None;
                    }

                    //Valida a todos los usuarios que esten en la sala  
                    for usuario in &usernames {
                        if !usuarios.contains_key(usuario) {
                            return Some(MensajesDeSalida::RESPONSE {
                                operation: Operacion::INVITE,
                                resultado: ResultadoOperacion::NO_SUCH_USER,
                                extra: Some(usuario.clone()),
                            });
                        }
                    }

                    //Crea la invitacion para los destinatario del chat
                    let invitacion = MensajesDeSalida::INVITATION {
                        username: emisor.clone(),
                        roomname: roomname.clone(),
                    };

                    for usuario in usernames {
                        //Por si hay algún duplicado o si ya está en la sala
                        if sala.miembros.contains(&usuario) || sala.invitados.contains(&usuario){
                            continue;
                        }

                        sala.invitados.insert(usuario.clone());

                        if let Some((tx_destino, _)) = usuarios.get(&usuario){
                            let _ = tx_destino.send(invitacion.clone());
                        }
                    }

                    None
                }

                MensajesDeEntrada::JOIN_ROOM { roomname } => {
                    info!("Se acepto la invitación a: {}", roomname);

                    //Se tiene que validar la invitación y unir a la sala

                    //Se verifica que este el emisor
                    let emisor: String = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(e) => return Some(e), 
                    };

                    let mut memoria = estado.lock().await;
                    let EstadoServidor { ref usuarios, ref mut salas } = *memoria;

                    //Valida que exista la sala
                    let sala = match salas.get_mut(&roomname){
                        Some(s) => s,
                        None => {
                            return Some(MensajesDeSalida::RESPONSE {
                                operation: Operacion::JOIN_ROOM,
                                resultado: ResultadoOperacion::NO_SUCH_ROOM,
                                extra: Some(roomname),
                            });
                        }
                    };

                    //validar que el usuario haya sido invitado
                    if !sala.invitados.contains(&emisor){
                        return Some(MensajesDeSalida::RESPONSE {
                            operation: Operacion::JOIN_ROOM,
                            resultado: ResultadoOperacion::NOT_INVITED,
                            extra: Some(roomname),
                        });
                    }

                    //cambiamos los parámetros de invitados y de miembros
                    sala.invitados.remove(&emisor);
                    sala.miembros.insert(emisor.clone());

                    let msj_notificacion = MensajesDeSalida::JOINED_ROOM {
                        roomname: roomname.clone(),
                        username: emisor.clone(),
                    };

                    for miembro in &sala.miembros {
                        if let Some((tx_destino, _estado_usuario)) = usuarios.get(miembro) {
                            let _ = tx_destino.send(msj_notificacion.clone());
                        }
                    }

                    Some(MensajesDeSalida::RESPONSE {
                        operation: Operacion::JOIN_ROOM,
                        resultado: ResultadoOperacion::SUCCESS,
                        extra: Some(roomname),
                    });

                    None
                }

                MensajesDeEntrada::ROOM_USERS { roomname } => {
                    info!("Se solicita la lista de usuarios en la sala {}", roomname);

                    //Muestra el HashMap de la lista de los usuarios de dicha sala

                    let emisor: String = match verificar_usuario(nombre_actual){
                        Ok(nombre) => nombre,
                        Err(e)=> return Some(e),
                    };

                    let memoria = estado.lock().await;

                    let EstadoServidor { ref usuarios, ref salas } = *memoria;

                    //Valida que la sala exista 
                    let sala = match salas.get(&roomname) {
                        Some(s) => s,
                        None => {
                            return Some(MensajesDeSalida::RESPONSE {
                                operation: Operacion::ROOM_USERS,
                                resultado: ResultadoOperacion::NO_SUCH_ROOM,
                                extra: Some(roomname),
                            })
                        }
                    };

                    //valida que el usuario ya este en la parte de miembros 
                    if !sala.miembros.contains(&emisor) {
                        return Some(MensajesDeSalida::RESPONSE {
                            operation: Operacion::ROOM_USERS,
                            resultado: ResultadoOperacion::NOT_JOINED,
                            extra: Some(roomname.clone()),
                        })
                    }

                    let mut diccionario_usuarios = std::collections::HashMap::new();
                    for miembro in &sala.miembros {
                        if let Some((_tx, estado_usuario)) = usuarios.get(miembro){
                            diccionario_usuarios.insert(miembro.clone(), format!("{:?}", estado_usuario));
                        }
                    }

                    //Devuelve la lista de ususarios
                    Some(MensajesDeSalida::ROOM_USER_LIST {
                        roomname: roomname,
                        users: diccionario_usuarios,
                    })

                    
                }

                MensajesDeEntrada::ROOM_TEXT { roomname, text } => {
                    info!("Se manda un msj a la sala {}: {}", roomname, text);

                    //Enviar el texto a todos los usuarios de la sala
                    None
                }

                MensajesDeEntrada::LEAVE_ROOM { roomname } => {
                    info!("Abandonó la sala: {}", roomname);

                    //Debe de salir de la sala el usuario
                    None
                }

                MensajesDeEntrada::DISCONNECT => {
                    info!("Se solicitó una desconexión");

                    //Limpiar al usuario del HashMap y mandar la noti de DISCONNECTED
                    None
                }
            }
        }

        Err(e) => {
            error!("Comando invalido del JSON: {}", e);
            
            //Debe de salir INVALID si llega a pasar algo que no
            Some(MensajesDeSalida::RESPONSE{
                operation: Operacion::INVALID,
                resultado: ResultadoOperacion::INVALID,
                extra: None,
            })
        }
    }
}

fn verificar_usuario(nombre_actual: &mut Option<String>) -> Result<String, MensajesDeSalida>{
    match nombre_actual{
        Some(nombre) => Ok(nombre.clone()),
        None => Err(MensajesDeSalida::RESPONSE {
            operation: Operacion::INVALID,
            resultado: ResultadoOperacion::INVALID,
            extra: None,
        }),
    }
}

#[cfg(test)]
mod test{
    use crate::estado::EstadoServidor;
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

}
