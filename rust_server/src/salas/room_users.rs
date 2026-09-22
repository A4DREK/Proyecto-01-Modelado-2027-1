use crate::estado::EstadoServidor;
use crate::protocolo::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn procesar(
    estado: &Arc<Mutex<EstadoServidor>>,
    roomname: String,
    emisor: String,
) -> Option<MensajesDeSalida> {
    let memoria = estado.lock().await;

    let EstadoServidor {
        ref usuarios,
        ref salas,
    } = *memoria;

    //Valida que la sala exista
    let sala = match salas.get(&roomname) {
        Some(s) => s,
        None => {
            return Some(MensajesDeSalida::RESPONSE {
                operation: Operacion::ROOM_USERS,
                resultado: ResultadoOperacion::NO_SUCH_ROOM,
                extra: Some(roomname),
            });
        }
    };

    //valida que el usuario ya este en la parte de miembros
    if !sala.miembros.contains(&emisor) {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::ROOM_USERS,
            resultado: ResultadoOperacion::NOT_JOINED,
            extra: Some(roomname.clone()),
        });
    }

    let mut diccionario_usuarios = std::collections::HashMap::new();
    for miembro in &sala.miembros {
        if let Some((_tx, estado_usuario)) = usuarios.get(miembro) {
            diccionario_usuarios.insert(miembro.clone(), format!("{:?}", estado_usuario));
        }
    }

    //Devuelve la lista de ususarios
    Some(MensajesDeSalida::ROOM_USER_LIST {
        roomname,
        users: diccionario_usuarios,
    })
}
