use crate::estado::EstadoServidor;
use crate::protocolo::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn procesar(
    estado: &Arc<Mutex<EstadoServidor>>,
    emisor: String,
    destinatario: String,
    texto: String,
) -> Option<MensajesDeSalida> {
    //Lector de la memoria
    let memoria = estado.lock().await;

    //Buscamos al usuario que este en el HashMap
    match memoria.usuarios.get(&destinatario) {
        Some((tx_destino, _estado)) => {
            let msj = MensajesDeSalida::TEXT_FROM {
                username: emisor,
                text: texto.clone(),
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
