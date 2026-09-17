//Ahorita solo serán los mensajes de entrada, a un no muestra nda de msj de salida
use crate::protocolo::{MensajesDeEntrada, MensajesDeSalida, Operacion, ResultadoOperacion, EstadoUsuario};
use crate::estado::{EstadoCompartido, Transmisor};
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
                //Inicia a checar que parte del comando del JSON es, vamos por casos
                //Solo POR ESTE MOMENTO ES PARA QUE VER SI SI JALA O NO 
                //falta el poder almacenar los usuarios en el hashmap y así lol
                MensajesDeEntrada::IDENTIFY{username} => {
                    info!("Nombre del cliente: {}", username);

                    //Modificador de la memoria
                    let mut memoria =  estado.lock().await;

                    //Ver si el nombre no existe
                    if memoria.usuarios.contains_key(&username){
                        return Some(MensajesDeSalida::RESPONSE {
                            operation: Operacion::IDENTIFY,
                            resultado: ResultadoOperacion::USER_ALREADY_EXISTS,
                            extra: Some("El usuario ya existe bro".to_string()) 
                        });
                    }

                    //Si no existe 
                    memoria.usuarios.insert(
                        username.clone(),
                        (tx_cliente.clone(),EstadoUsuario::ACTIVE )
                    );

                    *nombre_actual = Some(username.clone());
                    

                    //Retorno del mensaje
                    Some(MensajesDeSalida::RESPONSE {
                        operation: Operacion::IDENTIFY,
                        resultado: ResultadoOperacion::SUCCESS,
                        extra: Some(username),
                    })
                }

                MensajesDeEntrada::STATUS { status } => {
                    info!("Cambio de estado del usuario a: {:?}", status);
                    //Aquí se debe de actualizar el estado en memoria y pasarlo a todos los usuarios
                    None
                }

                MensajesDeEntrada::USERS => {
                    info!("Se solicita la lista de usuarios");
                    //LEER EL FKING HASH MAP Y DEVOLVER USER_LIST   
                    None
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

#[cfg(test)]
mod test{
    use crate::estado::EstadoServidor;
    use super::*;
    use tokio::sync::{mpsc, Mutex};
    use std::sync::Arc;
    use std::collections::HashMap;

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

}
