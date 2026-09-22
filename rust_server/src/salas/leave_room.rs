use crate::estado::EstadoServidor;
use crate::protocolo::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn procesar(
    estado: &Arc<Mutex<EstadoServidor>>,
    roomname: String,
    emisor: String,
) -> Option<MensajesDeSalida> {
    let mut memoria = estado.lock().await;
    let EstadoServidor {
        ref usuarios,
        ref mut salas,
    } = *memoria;

    //Validar que existe sala
    let sala = match salas.get_mut(&roomname) {
        Some(s) => s,
        None => {
            return Some(MensajesDeSalida::RESPONSE {
                operation: Operacion::LEAVE_ROOM,
                resultado: ResultadoOperacion::NO_SUCH_ROOM,
                extra: Some(roomname),
            });
        }
    };

    if !sala.miembros.contains(&emisor) {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::LEAVE_ROOM,
            resultado: ResultadoOperacion::NOT_JOINED,
            extra: Some(roomname),
        });
    }

    sala.miembros.remove(&emisor);

    let msj_sala = MensajesDeSalida::LEFT_ROOM {
        roomname: roomname.clone(),
        username: emisor.clone(),
    };

    for miembro in &sala.miembros {
        if let Some((tx_destino, _)) = usuarios.get(miembro) {
            let _ = tx_destino.send(msj_sala.clone());
        }
    }

    None
}
