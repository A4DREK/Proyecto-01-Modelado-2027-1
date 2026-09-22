//Codigo que me robé de: https://github.com/dheerajgopi/nimblecache/tree/blog-1
use crate::{
    estado::EstadoCompartido,
    manejador,
    protocolo::{MensajesDeSalida, ResultadoOperacion},
    usuarios,
};

use anyhow::{Error, Result};
use futures::{SinkExt, StreamExt};
use log::{error, info};
use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    signal,
    sync::broadcast,
};
use tokio_util::codec::{Framed, LinesCodec};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    estado: EstadoCompartido,
}

impl Server {
    pub fn new(listener: TcpListener, estado: EstadoCompartido) -> Server {
        Server { listener, estado }
    }

    pub async fn run(&mut self) -> Result<()> {
        let (shutdown_tx, _) = broadcast::channel::<()>(1);

        loop {
            tokio::select! {

                resultado_conexion = self.aceptar_conexion() => {

                    let socket =  match resultado_conexion {
                        Ok(stream) => stream,
                        Err(e) => {
                            error!("Error en la conexión :( {}", e);
                            continue;
                        }
                    };

                    let estado_cliente = Arc::clone(&self.estado);
                    let mut shutdown_rx = shutdown_tx.subscribe();

                    tokio::spawn(async move {
                        let mut framed: Framed<TcpStream, LinesCodec> = Framed::new(socket, LinesCodec::new());
                        let mut nombre_actual: Option<String> = None;
                        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

                        loop {
                            tokio::select! {
                                // Lectura de socket del cliente
                                resultado = framed.next() => {
                                    let linea = match resultado {
                                        Some(Ok(l)) => l,
                                        Some(Err(e)) => {
                                            error!("Error en la lectura: {}", e);
                                            break;
                                        }
                                        None => break, // El cliente se desconectó
                                    };
                                    info!("Recibido: {}", linea);

                                    let respuesta = match manejador::procesar_json(
                                        &linea,
                                        estado_cliente.clone(),
                                        tx.clone(),
                                        &mut nombre_actual).await {
                                            Some(r) =>  r,
                                            None => continue,
                                    };

                                    let salida_json = serde_json::to_string(&respuesta).unwrap_or_default();
                                    if let Err(e) = framed.send(salida_json).await {
                                        error!("Error al enviar msj al cliente: {}", e);
                                        break;
                                    }

                                    if let MensajesDeSalida::RESPONSE { ref resultado, .. } = respuesta
                                        && (*resultado == ResultadoOperacion::INVALID || *resultado == ResultadoOperacion::NOT_IDENTIFIED) {
                                            info!("Desconectando cliente por fallo de protocolo: {:?}", resultado);
                                            break;
                                        }

                                }

                                Some(mensaje_interno) = rx.recv() => {
                                    let salida_json = serde_json::to_string(&mensaje_interno).unwrap_or_default();
                                    if let Err(e) = framed.send(salida_json).await {
                                        error!("Error al enviar msj interno: {}", e);
                                        break;
                                    }
                                }

                                _ = shutdown_rx.recv() => {
                                    info!("Cerrando conexión con cliente por Graceful Shutdown...");
                                    let msj_cierre = r#"{"type":"DISCONNECT","reason":"Servidor apagándose"}"#;
                                    let _ = framed.send(msj_cierre.to_string()).await;
                                    break; // Rompe el loop y ejecuta procesar_disconnect
                                }
                            }
                        }

                        usuarios::procesar_disconnect(&estado_cliente, &mut nombre_actual).await;
                        info!("Cliente se desconectó :o");
                    });

                }

                _ = signal::ctrl_c() => {
                    info!("Señal cortada por ctrl c, se hará el cierre mmon");

                    let _ = shutdown_tx.send(());

                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

                    break;
                }
            }
        }
        Ok(())
    }

    async fn aceptar_conexion(&mut self) -> Result<TcpStream> {
        match self.listener.accept().await {
            Ok((socket, _)) => Ok(socket),
            Err(e) => Err(Error::from(e)),
        }
    }
}
