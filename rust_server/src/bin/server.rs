use tokio::net::TcpListener;

#[tokio::main]
async fn main(){
    let bind_addr = "0.0.0.0:1234";
    listen(bind_addr).await;
}

async fn listen(bind_addr: &str){
    let listener = TcpListener::bind(bind_addr).await.unwrap();
    loop{
        let(stream, _) = listener.accept().await.unwrap();
        println!("Connection from {}", stream.peer_addr().unwrap());
    }
}
