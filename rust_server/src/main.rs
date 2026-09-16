//Código robado de https://github.com/dheerajgopi/nimblecache/tree/blog-1

mod server;
mod manejador;
mod protocolo;
mod estado;

use crate::{estado::EstadoServidor, server::Server};
use clap::Parser;
use tokio::sync::Mutex;
use log::info;
use tokio::net::TcpListener;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(name = "Servidor Duckson", about = "Un server TCP")]
pub struct Cli{

    #[arg(short, long, default_value = "8080")]
    puerto: u16,

}


#[tokio::main]
async fn main() -> anyhow::Result<()>{

    //inicio de los logs en consola ocupar en la terminal RUST_LOG=info cargo run -- --(num puerto)
    env_logger::init();
    //Para los comandos de la terminal
    let args = Cli::parse();

    //Se crea la dirección para que el ususario en la terminal ponga de que 
    //cargo run -- --puerto 6767
    let direccion = format!("127.0.0.1:{}", args.puerto);


    let escucha  = match TcpListener::bind(&direccion).await{
        Ok(tcp_listener) => {
            info!("El escucha TCP inició en el puerto {}", direccion);
            tcp_listener
        },
        Err(e) => panic!("No se puede coneectar el escucha con {} .Err: {}", &direccion, e),
    };

    let estado_compartido = Arc::new(Mutex::new(EstadoServidor::nuevo()));
    //Inicia el server
    let mut server = Server::new(escucha, estado_compartido);

    server.run().await?;

    Ok(())
}
