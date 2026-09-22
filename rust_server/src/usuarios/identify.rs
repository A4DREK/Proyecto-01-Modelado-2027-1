use crate::estado::EstadoServidor;
use crate::protocolo::*;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

pub async fn procesar(
    estado: &Arc<Mutex<EstadoServidor>>,
    nombre_actual: &mut Option<String>,
    nuevo_usuario: String,
    tx_cliente: mpsc::UnboundedSender<MensajesDeSalida>,
) -> Option<MensajesDeSalida> {
    if nuevo_usuario.chars().count() > 8 {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::INVALID,
            resultado: ResultadoOperacion::INVALID,
            extra: None,
        });
    }

    //Modificador de la memoria
    let mut memoria = estado.lock().await;

    //Ver si el nombre no existe
    if memoria.usuarios.contains_key(&nuevo_usuario) {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::IDENTIFY,
            resultado: ResultadoOperacion::USER_ALREADY_EXISTS,
            extra: Some(nuevo_usuario),
        });
    }

    let msj_notificacion = MensajesDeSalida::NEW_USER {
        username: nuevo_usuario.clone(),
    };

    for (tx_destino, _estado) in memoria.usuarios.values() {
        let _ = tx_destino.send(msj_notificacion.clone());
    }

    //Si no existe
    memoria.usuarios.insert(
        nuevo_usuario.clone(),
        (tx_cliente.clone(), EstadoUsuario::ACTIVE),
    );

    *nombre_actual = Some(nuevo_usuario.clone());

    //Retorno del mensaje
    Some(MensajesDeSalida::RESPONSE {
        operation: Operacion::IDENTIFY,
        resultado: ResultadoOperacion::SUCCESS,
        extra: Some(nuevo_usuario),
    })
}
