//Ahorita solo serán los mensajes de entrada, a un no muestra nda de msj de salida
use crate::protocolo::{MensajesDeEntrada, MensajesDeSalida, Operacion, ResultadoOperacion};
use crate::estado::EstadoCompartido;
use log::{info, error};

pub async fn procesar_json(linea_txt : &str, estado: EstadoCompartido) -> Option<MensajesDeSalida>{

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

                MensajesDeEntrada::TEXT { username, text } => {
                    info!("Mensaje de {}: {}", username, text );
                    //Buscar el socket del destinatario y devovler TEXT_FROM
                    None
                }

                MensajesDeEntrada::PUBLIC_TEXT { text } => {
                    info!("Mensaje general: {}", text);
                    // Enviar el texto a todos los usuarios que esten en la red
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
