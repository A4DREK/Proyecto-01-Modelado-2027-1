
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
    if nombre_actual.is_some() {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::INVALID,
            resultado: ResultadoOperacion::INVALID,
            extra: None,
        });
    }

    let usuario_limpio = nuevo_usuario.trim().to_string();

    if usuario_limpio.chars().count() > 8 || usuario_limpio.is_empty() {
        //REVISAR SI EL USUARIO YA SE IDENTIFICÓ
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::INVALID,
            resultado: ResultadoOperacion::INVALID,
            extra: None,
        });
    }

    //Modificador de la memoria
    let mut memoria = estado.lock().await;

    //Ver si el nombre no existe
    if memoria.usuarios.contains_key(&usuario_limpio) {
        return Some(MensajesDeSalida::RESPONSE {
            operation: Operacion::IDENTIFY,
            resultado: ResultadoOperacion::USER_ALREADY_EXISTS,
            extra: Some(usuario_limpio),
        });
    }

    let msj_notificacion = MensajesDeSalida::NEW_USER {
        username: usuario_limpio.clone(),
    };

    for (tx_destino, _estado) in memoria.usuarios.values() {
        let _ = tx_destino.send(msj_notificacion.clone());
    }

    //Si no existe
    memoria.usuarios.insert(
        usuario_limpio.clone(),
        (tx_cliente.clone(), EstadoUsuario::ACTIVE),
    );

    *nombre_actual = Some(usuario_limpio.clone());

    //Retorno del mensaje
    Some(MensajesDeSalida::RESPONSE {
        operation: Operacion::IDENTIFY,
        resultado: ResultadoOperacion::SUCCESS,
        extra: Some(nuevo_usuario),
    })
}
