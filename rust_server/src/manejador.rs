use crate::protocolo::{MensajesDeEntrada, MensajesDeSalida};
use log::info::{info, error};

pub async fn procesar_json(linea_txt : &str){

    //Inicio de los casos para deserializar el JSON dsjf
    match serde::from_str<MensajesDeEntrada>(linea_txt){
        Ok(comando) => {
            info!("Coso del comando bien recibido: {:?}", comando);

            match comando{
                //Inicia a checar que parte del comando del JSON es, vamos por casos
                //Solo POR ESTE MOMENTO ES PARA QUE VER SI SI JALA O NO 
                //falta el poder almacenar los usuarios en el hashmap y así lol
                MensajesDeEntrada::IDENTIFY{username} => {
                    info!("Nombre del cliente: {}", username);
                }
                MensajeDeEntrada::TEXT{username, text} => {
                    info!("Msj de {}: {}", username, text);
                }
                MensajesDeEntrada::USERS => {
                    info!("Solicitud de la lista de usuarios");
                }
                _ => {
                    info!("Otros mensajes del protocolo");
                }
            }

        }

        Err(e) => {
            error!("Comando invalido del JSON en: {}. Texto recibido: {}", e, linea_txt);
        }
    }
}


