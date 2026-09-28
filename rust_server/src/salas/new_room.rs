use crate::estado::EstadoServidor;
use crate::estado::*;
use crate::protocolo::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn procesar(
    estado: &Arc<Mutex<EstadoServidor>>,
    roomname: String,
    emisor: String,
) -> Option<MensajesDeSalida> {
    let mut memoria = estado.lock().await;

    let roomname_limpio = roomname.trim();

    //validación de los caracteres y ""
    if roomname_limpio.chars().count() > 16 || roomname_limpio.is_empty() {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::INVALID,
            resultado: ResultadoOperacion::INVALID,
            extra: None,
        });
    }

    let roomname_final = roomname_limpio.to_string();

    //Si ya existe la sala
    if memoria.salas.contains_key(&roomname_final) {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::NEW_ROOM,
            resultado: ResultadoOperacion::ROOM_ALREADY_EXISTS,
            extra: Some(roomname_final),
        });
    }

    let mut miembros_iniciales = std::collections::HashSet::new();
    miembros_iniciales.insert(emisor.clone()); //Solo está el que creo la sala

    let nueva_sala = Salas {
        dueno_sala: emisor.clone(),
        miembros: miembros_iniciales,
        invitados: std::collections::HashSet::new(),
    };

    memoria.salas.insert(roomname_final.clone(), nueva_sala);

    Some(MensajesDeSalida::RESPONSE {
        operation: Operacion::NEW_ROOM,
        resultado: ResultadoOperacion::SUCCESS,
        extra: Some(roomname),
    })
}
