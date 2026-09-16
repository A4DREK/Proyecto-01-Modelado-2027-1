//Codigo que me robé de: https://github.com/dheerajgopi/nimblecache/tree/blog-1
use crate::manejador;

use anyhow::{Error, Result};
use log::{info, error};
use tokio::{net::{TcpListener, TcpStream},};
use tokio_util::codec::{Framed, LinesCodec};
use futures::{SinkExt, StreamExt};

#[derive(Debug)]
pub struct Server{
    listener: TcpListener,
}

impl Server{
    pub fn new(listener: TcpListener) -> Server{
        Server { listener }
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

            tokio::spawn(async move{

                //Convierte los sockets en un flujo de lineas para el JSON 
                let mut framed: Framed<TcpStream, LinesCodec> = Framed::new(socket, LinesCodec::new());

                //lectura de cada línea de código
                while let Some(resultado) = framed.next().await{
                    let linea = match resultado {
                        Ok(l) => l,
                        Err(e) => {
                            error!("Error en la lectura de la línea de código: {}", e);
                            break;
                        }
                        
                    };

                    info!("Recibido: {}", linea);

                    let respuesta = match manejador::procesar_json(&linea).await {
                        Some(r) => r,
                        None =>  continue,
                    };

                    let json_salida = match serde_json::to_string(&respuesta) {
                        Ok(j) => j,
                        Err(e) => {
                            error!("Error en la respuesta JSON: {} ", e);
                            continue;
                        }
                    };

                    if let Err(e) = framed.send(json_salida).await {
                        error!("Error al enviar msj al cliente: {}", e);
                        break;
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

