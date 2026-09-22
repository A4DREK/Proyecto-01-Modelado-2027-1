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

    //Valida que exista la sala
    let sala = match salas.get_mut(&roomname) {
        Some(s) => s,
        None => {
            return Some(MensajesDeSalida::RESPONSE {
                operation: Operacion::JOINED_ROOM,
                resultado: ResultadoOperacion::NO_SUCH_ROOM,
                extra: Some(roomname),
            });
        }
    };

    //validar que el usuario haya sido invitado
    if !sala.invitados.contains(&emisor) {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::JOINED_ROOM,
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
        operation: Operacion::JOINED_ROOM,
        resultado: ResultadoOperacion::SUCCESS,
        extra: Some(roomname),
    })
}
