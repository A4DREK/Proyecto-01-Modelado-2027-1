use crate::protocolo::*;
use crate::EstadoServidor;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn procesar(
    estado: &Arc<Mutex<EstadoServidor>>,
    emisor: String,
    texto: String,
) -> Option<MensajesDeSalida> {

    //Lector de la memomria
    let memoria = estado.lock().await;

    //Manda el msj a todos menos al emisor, usa un for que aquí es un iterador
    for (destinatario, (tx_destino, _estado)) in memoria.usuarios.iter(){
        if destinatario != &emisor{
            let msj = MensajesDeSalida::PUBLIC_TEXT_FROM {
                username: emisor.clone(),
                text: texto.clone(),
            };

            //Se manda el msj al otro usuario
            let _ = tx_destino.send(msj);
        }
    }

    None
}