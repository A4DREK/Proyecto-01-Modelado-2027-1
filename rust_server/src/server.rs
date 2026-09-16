//Codigo que me robé de: https://github.com/dheerajgopi/nimblecache/tree/blog-1
use crate::{estado::EstadoCompartido, manejador};

use anyhow::{Error, Result};
use log::{info, error};
use tokio::{net::{TcpListener, TcpStream},};
use tokio_util::codec::{Framed, LinesCodec};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;

#[derive(Debug)]
pub struct Server{
    listener: TcpListener,
    estado: EstadoCompartido, 
}

impl Server{
    pub fn new(listener: TcpListener, estado: EstadoCompartido) -> Server{
        Server { listener , estado}
    }

    pub async fn run(&mut self) -> Result<()>{

        loop{

            let socket =  match self.aceptar_conexion().await {
                Ok(stream) => stream,

                Err(e) => {
                    error!("{}", e);
                    panic!("Error en la conexión :(");
                }
            };

            let estado_cliente = Arc::clone(&self.estado);
            tokio::spawn(async move{

                //Convierte los sockets en un flujo de lineas para el JSON 
                let mut framed: Framed<TcpStream, LinesCodec> = Framed::new(socket, LinesCodec::new());
                let mut nombre_actual: Option<String> = None;
                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

                //lectura de cada línea de código
                loop {

                    //uso de TokioSelect 
                    tokio::select! {
                        resultado = framed.next() =>{
                            let linea = match resultado{
                                Some(Ok(l)) => l,
                                Some(Err(e)) => {
                                    error!("Erroe en la lectura: {}", e);   
                                    break;
                                }
                                None => break, //Valio el cliente   
                            };
                            info!("Recibido: {}", linea);

                            let respuesta = match manejador::procesar_json(
                                &linea, estado_cliente.clone(),
                                tx.clone(),
                                &mut nombre_actual).await{
                                    Some(r) =>  r,
                                    None => continue,
                                };

                            let salida_json = serde_json::to_string(&respuesta).unwrap_or_default();
                            if let Err(e) = framed.send(salida_json).await{
                                error!("Error al enciar msj al cliente: {}", e);
                                break;
                            }

                        }
                        
                        //Otro msj de otro usuario desde el canal interno rx
                        Some(mensaje_interno) = rx.recv() => {
                            let salida_json = serde_json::to_string(&mensaje_interno).unwrap_or_default();
                            if let Err(e) = framed.send(salida_json).await{
                                error!("Error al enviar msj interno: {}", e);
                                break;
                            }
                        }
                        
                    }
                }
                info!("Cliente se desconectó lol");
            });
        }
    }

    async fn aceptar_conexion(&mut self) -> Result<TcpStream> {

        loop{
            match self.listener.accept().await{
                Ok((socket, _)) => return Ok(socket),
                Err(e) => return Err(Error::from(e)),
            }
        }
    }
}

