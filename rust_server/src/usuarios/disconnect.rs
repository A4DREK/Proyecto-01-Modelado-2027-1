use crate::protocolo::*;
use crate::estado::EstadoServidor;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn procesar(
    estado: &Arc<Mutex<EstadoServidor>>,
    nombre_actual: &mut Option<String>,
) -> Option<MensajesDeSalida> {
    
    let emisor = match nombre_actual.take() {
        Some(nombre) => nombre,
        None => return None,
    };

    let mut memoria = estado.lock().await;
    //Ahora si ponemos como mut a ambos usuarios y salas pq lo vamos a mandar lejitos al usuario 
    let EstadoServidor { ref mut usuarios, ref mut salas } = *memoria;

    usuarios.remove(&emisor);

    for(roomname, sala) in salas.iter_mut() {
         if sala.miembros.remove(&emisor) {
            let msj_ida = MensajesDeSalida::LEFT_ROOM {
                roomname: roomname.clone(),
                username: emisor.clone(),
            };

            for miembro in &sala.miembros {
                if let Some((tx_destino, _)) = usuarios.get(miembro) {
                    let _ = tx_destino.send(msj_ida.clone());
                }
            }
         }

         sala.invitados.remove(&emisor);
    }

    let msj_desconectado = MensajesDeSalida::DISCONNECTED {
        username: emisor,
    };

    for(_nombre, (tx_destino, _)) in usuarios.iter() {
        let _ = tx_destino.send(msj_desconectado.clone());
    }

    None
}