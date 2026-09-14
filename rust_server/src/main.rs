//Código robado de https://github.com/dheerajgopi/nimblecache/tree/blog-1

mod server;

use crate::server::Server;
use clap::Parser;
use log::info;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(name = "Servidor Duckson", about = "Un server TCP")]
pub struct Cli{

    //Direccion predeterminada de la deirección IP 
    #[arg(short, long, default_value = "127.0.0.1")]
    addr: String,

    #[arg(short, long, default_value = "1234")]
    puerto: u16,
}


#[tokio::main]
async fn main() -> anyhow::Result<()>{

    env_logger::init();
    let args = Cli::parse();

    //Se crea la dirección para que el ususario en la terminal ponga de que 
    //cargo run -- --puerto 6767
    let direccion = format!("{}:{}", args.addr, args.puerto);


    let escucha  = match TcpListener::bind(&direccion).await{
        Ok(tcp_listener) => {
            info!("El escucha TCP inició en el puerto 6379");
            tcp_listener
        },
        Err(e) => panic!("No se puede coneectar el escucha con {} .Err: {}", &direccion, e),
    };

    //Inicia el server
    let mut server = Server::new(escucha);

    server.run().await?;

    Ok(())
}
