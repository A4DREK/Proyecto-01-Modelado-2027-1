use crate::protocolo::*;
use crate::EstadoServidor;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn procesar(
    estado: &Arc<Mutex<EstadoServidor>>,
    nombre_actual: &Option<String>,
    nuevo_estado: EstadoUsuario,
) -> Option<MensajesDeSalida> {

    //Memoria para buscar al usuario
    let mut memoria = estado.lock().await;

    //Se busca que esté identificó al usuario
    let emisor: String = match nombre_actual {
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