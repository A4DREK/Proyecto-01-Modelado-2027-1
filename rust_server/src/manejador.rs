use std::collections::HashMap;
use crate::protocolo::{MensajesDeEntrada, MensajesDeSalida, Operacion, ResultadoOperacion, EstadoUsuario};
use crate::estado::{EstadoCompartido, Salas, Transmisor, EstadoServidor};
use crate::usuarios;

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
                    usuarios::procesar_identify(&estado, nombre_actual, nuevo_usuario, tx_cliente.clone()).await
                }

                MensajesDeEntrada::STATUS { status: nuevo_estado } => {
                    info!("Cambio de estado del usuario a: {:?}", nuevo_estado);
                    
                    //Aquí se debe de actualizar el estado en memoria y pasarlo a todos los usuarios
                    usuarios::procesar_status(&estado, &nombre_actual, nuevo_estado).await
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

                MensajesDeEntrada::TEXT { username: destinatario, text } => {
                    info!("Mensaje para {}: {}", destinatario, text );
                    //Buscar el socket del destinatario y devovler TEXT_FROM
                    
                    let emisor = obtener_emisor(nombre_actual)?;
                    usuarios::procesar_text(&estado, emisor, destinatario, text).await
                }

                MensajesDeEntrada::PUBLIC_TEXT { text } => {
                    info!("Mensaje general: {}", text);
                    // Enviar el texto a todos los usuarios que esten en la red
                    let emisor  = obtener_emisor(nombre_actual)?;
                    usuarios::procesar_public_text(&estado, emisor, text).await

                    
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
                    let emisor: String = match verificar_usuario(nombre_actual) {
                        Ok(nombre) => nombre,
                        Err(e) => return Some(e),
                    };

                    let memoria = estado.lock().await;
                    let EstadoServidor { ref usuarios, ref salas } = *memoria;

                    //La sala existe?
                    let sala = match salas.get(&roomname) {
                        Some(s) => s,
                        None => {
                            return Some(MensajesDeSalida::RESPONSE {
                                operation: Operacion::ROOM_TEXT,
                                resultado: ResultadoOperacion::NO_SUCH_ROOM,
                                extra: Some(roomname),
                            });
                        }
                        
                    };

                    //Validar que el usuario esté en la sala
                    if !sala.miembros.contains(&emisor){
                        return Some(MensajesDeSalida::RESPONSE {
                            operation: Operacion::ROOM_TEXT,
                            resultado: ResultadoOperacion::NOT_JOINED,
                            extra: Some(roomname.clone()),
                        })
                    }

                    //Mensaje que se mandará a la sala
                    let msj_sala = MensajesDeSalida::ROOM_TEXT_FROM {
                        roomname: roomname.clone(),
                        username: emisor.clone(),
                        text,
                    };

                    for miembro in &sala.miembros {
                        if miembro != &emisor {
                            if let Some((tx_destino, _)) = usuarios.get(miembro) {
                                let _ = tx_destino.send(msj_sala.clone());
                            }
                        }
                    }


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

fn obtener_emisor(nombre_actual: &Option<String>) -> Option<String>{
    match nombre_actual {
        Some(nombre) => Some(nombre.clone()),
        None => {
            log::error!("Un usuario no identificado quiere mandar un msj");
            None
        }
    }
}

#[cfg(test)]
#[path ="pruebas_unitarias.rs"]
mod tests;

