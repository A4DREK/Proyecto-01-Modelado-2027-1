//ESTE CLIENTE ES SOLAMENTE DE PRUEBA PORQUE QUE HUEVA ESTAR ESCRBIENDO EL JSON
//A CADA RATO PORQUE, POR QUÉ HARÍA ESO? SABES O SEA NO 

use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() {

    //se conecta con el server
    let stream = match TcpStream::connect("127.0.0.1:1234").await{
        Ok(r) => r,
        Err(e) => {
            println!("ERRRRROOOOOOOOR: {}", e);
            return;
        }  
    };

    //ESTO ES PARA EL CLIENTE DE PRUEBA 
    let mut framed = Framed::new(stream, LinesCodec::new());
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    println!("Escribe nombre de usuario: ");

    let username = if let Ok(Some(line)) = stdin.next_line().await{
        line.trim().to_string()
    }else{
        print!("Error escribiendo");
        return;
    };

    let iden_json = json!({
        "type":"IDENTIFY",
        "username": username
    }).to_string();
    
    if framed.send(iden_json).await.is_err(){
        println!("ERRRRROOOOOOOOR");
        return;
    }

    println!("Mandá mensajes cawn");

    loop{
        tokio::select! {
            //Escrbir en la CLI
            linea_teclado = stdin.next_line() => {
                match linea_teclado {
                    Ok(Some(texto)) => {
                        let texto = texto.trim();
                        if !texto.is_empty(){

                            //Hace el parseo al JSON
                            let public_json = json!({
                                "type": "PUBLIC_TEXT",
                                "text": texto
                            }).to_string();
                            
                            let _ = framed.send(public_json).await;
                        }
                    }
                    _ => break,
                }
            }

            //Recibir msj del server

            msj_server = framed.next() => {
                match msj_server {
                    Some (Ok(json_str)) => {
                        println!(">> {}", json_str);
                    }
                    _ => {
                        println!("Pal Loby del server");
                        break;
                    }
                }
            }
        }
    }

}