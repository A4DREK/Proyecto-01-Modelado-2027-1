use tokio::net::TcpStream;

#[tokio::main]
async fn main(){
    let server_addr = "127.0.0.1.7878";
    let stream: TcpStream = TcpStream::connect(server_addr).await.unwrap();
    print!("Connected to {}", stream.peer_addr().unwrap());
}