//ESTE CLIENTE ES SOLAMENTE DE PRUEBA PORQUE QUE HUEVA ESTAR ESCRBIENDO EL JSON
//A CADA RATO PORQUE, POR QUÉ HARÍA ESO? SABES O SEA NO

use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::io::{self, AsyncBufReadExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
async fn main() {
    //creación de un parámetro para poder mandar arguemntos a consola
    let args: Vec<String> = std::env::args().collect();

    let server_adrr = if args.len() > 1 {
        args[1].clone()
    } else {
        print!("No se brindó un dirección IP, Default a 127.0.0.1:1234");
        "127.0.0.1:1234".to_string()
    };

    print!("Conexión a {}", server_adrr);

    //se conecta con el server
    let stream = match TcpStream::connect(&server_adrr).await {
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

    let username = if let Ok(Some(line)) = stdin.next_line().await {
        line.trim().to_string()
    } else {
        print!("Error escribiendo");
        return;
    };

    let iden_json = json!({
        "type":"IDENTIFY",
        "username": username
    })
    .to_string();

    if framed.send(iden_json).await.is_err() {
        println!("ERRRRROOOOOOOOR");
        return;
    }

    println!("Mandá mensajes cawn");

    loop {
        tokio::select! {
            //Escrbir en la CLI
            linea_teclado = stdin.next_line() => {
                match linea_teclado {
                    Ok(Some(texto)) => {
                        let texto = texto.trim();
                        if !texto.is_empty(){
                            let json_a_enviar = if texto == "/users" {
                                json!({
                                    "type": "USERS"
                                }).to_string()

                            } else if texto.starts_with("/status ") {
                                let estado = texto.trim_start_matches("/status ").trim();
                                json!({
                                    "type": "STATUS",
                                    "status": estado
                                }).to_string()

                            } else if texto.starts_with("/msg ") {
                                // Separa el texto en 3 partes máximas: "/msg", "Usuario", "El resto del mensaje"
                                let partes: Vec<&str> = texto.splitn(3, ' ').collect();
                                if partes.len() == 3 {
                                    json!({
                                        "type": "TEXT",
                                        "username": partes[1],
                                        "text": partes[2]
                                    }).to_string()
                                } else {
                                    println!("Formato incorrecto. Usa: /msg <usuario> <texto>");
                                    continue; // Salta esta iteración y espera nuevo texto
                                }

                            } else if texto.starts_with('/') {
                                println!("Comando desconocido. Comandos válidos: /users, /status <ESTADO>, /msg <user> <texto>");
                                continue;

                            } else {
                                // Si no tiene '/', asumimos que es un mensaje público
                                json!({
                                    "type": "PUBLIC_TEXT",
                                    "text": texto
                                }).to_string()
                            };

                            // Envía el JSON formateado usando tu Framed
                            if framed.send(json_a_enviar).await.is_err() {
                                println!("Error al enviar el mensaje al servidor");
                            }

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
