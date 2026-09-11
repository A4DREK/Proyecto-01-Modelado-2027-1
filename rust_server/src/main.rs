//Código robado de https://github.com/dheerajgopi/nimblecache/tree/blog-1

mod server;

use crate::server::Server;
use anyhow::Result;
use log::info;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()>{
    env_logger::init();

    let direccion = format!("127.0.0.1:{}", 6379);

    let escucha  = match TcpListener::bind(&direccion).await{
        Ok(tcp_listener) => {
            info!("El escucha TCP inició en el puerto 6379");
            tcp_listener
        },
        Err(e) => panic!("No se puede coneectar el escucha con {} .Err: {}", &direccion, e),
    };

    let mut server = Server::new(escucha);

    server.run().await?;

    Ok(())
}
