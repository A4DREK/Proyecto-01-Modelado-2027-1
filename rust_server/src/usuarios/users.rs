use crate::estado::EstadoServidor;
use crate::protocolo::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn procesar(estado: &Arc<Mutex<EstadoServidor>>) -> Option<MensajesDeSalida> {
    let memoria = estado.lock().await;

    let mut lista_usuarios: HashMap<String, String> = HashMap::new();

    for (nombre, (_tx, estado)) in memoria.usuarios.iter() {
        let estado_str = format!("{:?}", estado);
        lista_usuarios.insert(nombre.clone(), estado_str);
    }

    Some(MensajesDeSalida::USER_LIST {
        users: lista_usuarios,
    })
}
