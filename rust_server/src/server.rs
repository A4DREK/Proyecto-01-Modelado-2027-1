//Codigo que me robé de: https://github.com/dheerajgopi/nimblecache/tree/blog-1

use anyhow::{Error, Result};
use log::error;
use tokio::{io::AsyncWriteExt,  
            net::{TcpListener, TcpStream},};

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
                if let Err(e) = &mut socket.write_all("Hola! ".as_bytes()).await{
                    error!("{}", e);
                    panic!("Error escribiendo la respuesta")
                }
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
