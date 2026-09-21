use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
//use tokio_util::bytes::buf::{self, Reader};
use std::time::{Instant, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;
use rust_server::server::Server;
use rust_server::estado::EstadoServidor;

const NUM_CLIENTES: usize = 300;
const MENSAJES_POR_CLIENTE: usize = 5;

// Usamos el modo multi_thread para que la prueba soporte cientos de conexiones concurrentes
//Vi que eso era posible por este video je: https://www.youtube.com/watch?v=gCM2l3Z-yM8

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_stress_servidor() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Info) // 👈 LÍNEA CLAVE
        .try_init();

    // Configurar el estado global
    let estado = Arc::new(Mutex::new(EstadoServidor::nuevo()));

    // Levantar el servidor en un puerto aleatorio 
    // Se escogio el 0 por: https://cubicspot-blogspot-com.translate.goog/2016/04/need-random-tcp-port-number-for-your.html?_x_tr_sch=http&_x_tr_sl=en&_x_tr_tl=es&_x_tr_hl=es&_x_tr_pto=sge
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("Fallo al hacer bind");
    let puerto_asignado = listener.local_addr().unwrap().port();
    let host = format!("127.0.0.1:{}", puerto_asignado);
    
    let mut servidor = Server::new(listener, estado);

    // Ejecutar el servidor en una tarea de fondo (background)
    tokio::spawn(async move {
        if let Err(e) = servidor.run().await {
            panic!("Error crítico en el servidor: {}", e);
        }
    });

    // Pequeña pausa para asegurar que el servidor está escuchando
    tokio::time::sleep(Duration::from_millis(500)).await;

    println!("Iniciando prueba hacia {} con {} clientes...", host, NUM_CLIENTES);
    let inicio = Instant::now();

    // Lanzar los clientes concurrentes
    let mut tareas = vec![];
    for id in 0..NUM_CLIENTES {
        let host_clon = host.clone();
        let tarea = tokio::spawn(async move {
            ejecutar_cliente(id, host_clon).await
        });
        tareas.push(tarea);
    }

    // Recolectar resultados
    let mut exitosos = 0;
    for tarea in tareas {
        if let Ok(true) = tarea.await {
            exitosos += 1;
        }
    }

    let duracion = inicio.elapsed();
    let fallidos = NUM_CLIENTES - exitosos;

    println!("\n--- RESULTADOS DE ESTRÉS ---");
    println!("Tiempo: {:.2?}", duracion);
    println!("Exitosos: {}", exitosos);
    println!("Fallidos: {}", fallidos);

    // final de la prueba
    assert_eq!(fallidos, 0, "Hubo conexiones que fallaron durante la prueba de estrés");
}

async fn ejecutar_cliente(id: usize, host: String) -> bool {
    let username = format!("u_{:03}", id);
    let roomname = "sala_test";
    
    let socket = match TcpStream::connect(&host).await {
        Ok(s) => s,
        Err(e) => {
            println!("Error aquiiiii, al conectar el cliente {}: {} ", id, e);
            return false;
        }
    };

    let (mut lector, mut escritor) = socket.into_split();

    tokio::spawn(async move {
        let mut buf = [0; 2048];
        while let Ok(n) = lector.read(&mut buf).await{
            if n == 0 {
                break;
            }
        }
    });

    macro_rules! enviar {
        ($paso:expr, $json:expr) => {{
            let msj = format!("{}\n", $json);
            match escritor.write_all(msj.as_bytes()).await {
                Ok(_) => true,
                Err(e) => {
                    println!("errooor de [Cliente {}]en {}: {}",id, $paso, e);
                    false
                }
            }
        }};
    }

    if !enviar!("IDENTIFY", &format!(r#"{{"type":"IDENTIFY","username":"{}"}}"#, username)) {
        return false;
    } 
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    if !enviar!("JOINED_ROOM", &format!(r#"{{"type":"JOINED_ROOM","roomname":"{}"}}"#, roomname)) { 
        return false;
    }

    for i in 0..MENSAJES_POR_CLIENTE {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let msg_sala = format!(
            r#"{{"type":"ROOM_TEXT","roomname":"{}","text":"Mensaje {}"}}"#, 
            roomname, i
        );
        if !enviar!("ROOM_TEXT", &msg_sala) { return false; }
    }

    let _ = !enviar!("DISCONNECT", r#"{"type": "DISCONNECT"}"#);

    tokio::time::sleep(Duration::from_millis(50)).await;
    true
}

