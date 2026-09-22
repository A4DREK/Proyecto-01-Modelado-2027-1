use crate::estado::{EstadoCompartido, Transmisor};
use crate::protocolo::{MensajesDeEntrada, MensajesDeSalida, Operacion, ResultadoOperacion};
use crate::{salas, usuarios};

use log::{error, info};

pub async fn procesar_json(
    linea_txt: &str,
    estado: EstadoCompartido,
    tx_cliente: Transmisor,
    nombre_actual: &mut Option<String>,
) -> Option<MensajesDeSalida> {
    //Inicio de los casos para deserializar el JSON dsjf
    match serde_json::from_str::<MensajesDeEntrada>(linea_txt) {
        Ok(comando) => {
            info!("Coso del comando bien recibido: {:?}", comando);

            match comando {
                //Inicia a checar que parte del comando del JSON
                MensajesDeEntrada::IDENTIFY {
                    username: nuevo_usuario,
                } => {
                    info!("Nombre del cliente: {}", nuevo_usuario);
                    usuarios::procesar_identify(
                        &estado,
                        nombre_actual,
                        nuevo_usuario,
                        tx_cliente.clone(),
                    )
                    .await
                }

                MensajesDeEntrada::STATUS {
                    status: nuevo_estado,
                } => {
                    info!("Cambio de estado del usuario a: {:?}", nuevo_estado);

                    //Aquí se debe de actualizar el estado en memoria y pasarlo a todos los usuarios
                    usuarios::procesar_status(&estado, nombre_actual, nuevo_estado).await
                }

                MensajesDeEntrada::USERS => {
                    info!("Se solicita la lista de usuarios");
                    //LEER EL FKING HASH MAP Y DEVOLVER USER_LIST
                    let _emisor = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(mensaje_error) => return Some(mensaje_error),
                    };

                    usuarios::procesar_users(&estado).await
                }

                MensajesDeEntrada::TEXT {
                    username: destinatario,
                    text,
                } => {
                    info!("Mensaje para {}: {}", destinatario, text);
                    //Buscar el socket del destinatario y devovler TEXT_FROM

                    let emisor = obtener_emisor(nombre_actual)?;
                    usuarios::procesar_text(&estado, emisor, destinatario, text).await
                }

                MensajesDeEntrada::PUBLIC_TEXT { text } => {
                    info!("Mensaje general: {}", text);
                    // Enviar el texto a todos los usuarios que esten en la red
                    let emisor = obtener_emisor(nombre_actual)?;
                    usuarios::procesar_public_text(&estado, emisor, text).await
                }

                MensajesDeEntrada::NEW_ROOM { roomname } => {
                    info!("Se crea una nueva sala, llamada: {}", roomname);
                    //Tengo que meter la lógica para crear las salas

                    let emisor = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(e) => return Some(e),
                    };

                    salas::new_room::procesar(&estado, roomname, emisor).await
                }

                MensajesDeEntrada::INVITE {
                    roomname,
                    usernames,
                } => {
                    info!("Invitar de {} a {:?}", roomname, usernames);
                    //Algo tengo que hacer para las invitaciones

                    let emisor = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(e) => return Some(e),
                    };

                    salas::procesar_invite(&estado, roomname, usernames, emisor).await
                }

                MensajesDeEntrada::JOINED_ROOM { roomname } => {
                    info!("Se acepto la invitación a: {}", roomname);

                    //Se tiene que validar la invitación y unir a la sala

                    //Se verifica que este el emisor
                    let emisor: String = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(e) => return Some(e),
                    };

                    salas::procesar_join_room(&estado, roomname, emisor).await
                }

                MensajesDeEntrada::ROOM_USERS { roomname } => {
                    info!("Se solicita la lista de usuarios en la sala {}", roomname);

                    //Muestra el HashMap de la lista de los usuarios de dicha sala

                    let emisor: String = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(e) => return Some(e),
                    };

                    salas::procesar_room_users(&estado, roomname, emisor).await
                }

                MensajesDeEntrada::ROOM_TEXT { roomname, text } => {
                    info!("Se manda un msj a la sala {}: {}", roomname, text);

                    //Enviar el texto a todos los usuarios de la sala
                    let emisor: String = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(e) => return Some(e),
                    };

                    salas::procesar_room_text(&estado, roomname, text, emisor).await
                }

                MensajesDeEntrada::LEAVE_ROOM { roomname } => {
                    info!("Abandonó la sala: {}", roomname);

                    let emisor = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(e) => return Some(e),
                    };

                    salas::procesar_leave_room(&estado, roomname, emisor).await
                }

                MensajesDeEntrada::DISCONNECT => {
                    info!("Se solicitó una desconexión");

                    //Limpiar al usuario del HashMap y mandar la noti de DISCONNECTED
                    usuarios::procesar_disconnect(&estado, nombre_actual).await
                }
            }
        }

        Err(e) => {
            error!("Comando invalido del JSON: {}", e);

            //Debe de salir INVALID si llega a pasar algo que no
            Some(MensajesDeSalida::RESPONSE {
                operation: Operacion::INVALID,
                resultado: ResultadoOperacion::INVALID,
                extra: None,
            })
        }
    }
}

fn verificar_usuario(nombre_actual: &mut Option<String>) -> Result<String, MensajesDeSalida> {
    match nombre_actual {
        Some(nombre) => Ok(nombre.clone()),
        None => Err(MensajesDeSalida::RESPONSE {
            operation: Operacion::INVALID,
            resultado: ResultadoOperacion::INVALID,
            extra: None,
        }),
    }
}

fn obtener_emisor(nombre_actual: &Option<String>) -> Option<String> {
    match nombre_actual {
        Some(nombre) => Some(nombre.clone()),
        None => {
            log::error!("Un usuario no identificado quiere mandar un msj");
            None
        }
    }
}

#[cfg(test)]
#[path = "pruebas_unitarias.rs"]
mod tests;
