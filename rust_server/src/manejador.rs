use std::collections::HashMap;

//Ahorita solo serán los mensajes de entrada, a un no muestra nda de msj de salida
use crate::protocolo::{MensajesDeEntrada, MensajesDeSalida, Operacion, ResultadoOperacion, EstadoUsuario};
use crate::estado::{EstadoCompartido, Salas, Transmisor};
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
                    None
                }

                MensajesDeEntrada::JOIN_ROOM { roomname } => {
                    info!("Se acepto la invitación a: {}", roomname);

                    //Se tiene que validar la invitación y unir a la sala
                    None
                }

                MensajesDeEntrada::ROOM_USERS { roomname } => {
                    info!("Se solicita la lista de usuarios en la sala {}", roomname);

                    //Muestra el HashMap de la lista de los usuarios de dicha sala
                    None
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

    //Si el destinatario no existe
    #[tokio::test] 
    async fn test_no_existe_usuario(){
        let estado_mock = Arc::new(Mutex::new(EstadoServidor::nuevo()));

        let (tx_cliente, _rx_cliente) = mpsc::unbounded_channel();
        
        let mut nombre_actual = Some("Aly".to_string());
        
        let linea_txt = r#"{"type": "TEXT", "username": "Bob", "text": "Hola Bob"}"#.to_string();

        
        let respuesta = procesar_json(
            &linea_txt,
            estado_mock.clone(),
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

}
