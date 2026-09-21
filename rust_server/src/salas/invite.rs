use crate::protocolo::*;
use crate::estado::EstadoServidor;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn procesar(
    estado: &Arc<Mutex<EstadoServidor>>,
    roomname: String,
    usernames: Vec<String>,
    emisor: String,
) -> Option<MensajesDeSalida> {

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