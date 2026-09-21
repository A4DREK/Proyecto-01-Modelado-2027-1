use crate::protocolo::*;
use crate::EstadoServidor;
use crate::estado::*; 
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn procesar(
    estado: &Arc<Mutex<EstadoServidor>>,
    roomname: String,
    emisor: String,
) -> Option<MensajesDeSalida> {
    
    let mut memoria = estado.lock().await;

    if roomname.chars().count() > 16 {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::INVALID,
            resultado: ResultadoOperacion::INVALID,
            extra: None,
        });
    }

    //Si ya existe la sala 
    if memoria.salas.contains_key(&roomname){
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::NEW_ROOM,
            resultado: ResultadoOperacion::ROOM_ALREADY_EXISTS,
            extra: Some(roomname),
        });
    }

    let mut miembros_iniciales  = std::collections::HashSet::new();
    miembros_iniciales.insert(emisor.clone()); //Solo está el que creo la sala

    let nueva_sala = Salas {
        dueno_sala: emisor.clone(),
        miembros: miembros_iniciales,
        invitados: std::collections::HashSet::new(),
    };

    memoria.salas.insert(roomname.clone(), nueva_sala);

    Some(MensajesDeSalida::RESPONSE {
        operation: Operacion::NEW_ROOM,
        resultado: ResultadoOperacion::SUCCESS,
        extra: Some(roomname),
    })

}