use crate::estado::EstadoServidor;
use crate::protocolo::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn procesar(
    estado: &Arc<Mutex<EstadoServidor>>,
    roomname: String,
    text: String,
    emisor: String,
) -> Option<MensajesDeSalida> {
    let memoria = estado.lock().await;
    let EstadoServidor {
        ref usuarios,
        ref salas,
    } = *memoria;

    let roomname_limpio = roomname.trim();

    if roomname_limpio.is_empty() || text.trim().is_empty() {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::ROOM_TEXT,
            resultado: ResultadoOperacion::INVALID,
            extra: None,
        });
    }
    //La sala existe?
    let sala = match salas.get(&roomname) {
        Some(s) => s,
        None => {
            return Some(MensajesDeSalida::RESPONSE {
                operation: Operacion::ROOM_TEXT,
                resultado: ResultadoOperacion::NO_SUCH_ROOM,
                extra: Some(roomname_limpio.to_string()),
            });
        }
    };

    //Validar que el usuario esté en la sala
    if !sala.miembros.contains(&emisor) {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::ROOM_TEXT,
            resultado: ResultadoOperacion::NOT_JOINED,
            extra: Some(roomname_limpio.to_string()),
        });
    }

    //Mensaje que se mandará a la sala
    let msj_sala = MensajesDeSalida::ROOM_TEXT_FROM {
        roomname: roomname.clone(),
        username: emisor.clone(),
        text,
    };

    for miembro in &sala.miembros {
        if miembro != &emisor
            && let Some((tx_destino, _)) = usuarios.get(miembro)
        {
            let _ = tx_destino.send(msj_sala.clone());
        }
    }

    None
}
