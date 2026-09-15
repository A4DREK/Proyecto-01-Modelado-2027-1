//Codigo que me robé de: https://github.com/dheerajgopi/nimblecache/tree/blog-1

use crate::manejador;
use anyhow::{Error, Result};
use log::{info, error};
use tokio::{net::{TcpListener, TcpStream},};
use tokio_util::codec::{framed, LinesCodec};

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

            let mut socket =  match self.aceptar_conexion().await {
                Ok(stream) => stream,

                Err(e) => {
                    error!("{}", e);
                    panic!("Error en la conexión :(");
                }
            };

            tokio::spawn(async move{

                //Convierte los sockets en un flujo de lineas para el JSON 
                let mut framed = Framed::new(socket, LinesCodec::new());

                //lectura de cada línea de código
                while let Some(resultado) = framed.next().await {
                    match resultado{
                        Ok(linea) => {
                            manejador::procesar_json(&linea).await;
                        }
                        Err(e) => {
                            erorr!("Error en la lectura de la línea de código: {}", e);
                            break;
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

