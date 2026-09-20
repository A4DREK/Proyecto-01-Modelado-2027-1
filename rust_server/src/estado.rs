use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use crate::protocolo::{EstadoUsuario, MensajesDeSalida};

//Creación de un transmisor para mandar msj al socket del usuario 
pub type Transmisor = mpsc::UnboundedSender<MensajesDeSalida>;

//Para la parte de las salas
#[derive(Debug)]
pub struct Salas{
    pub dueno_sala: String,
    pub miembros: HashSet<String>,
    pub invitados: HashSet<String>, //para que no se repita
}

#[derive(Debug)]
pub struct EstadoServidor{
    //Llave1 del HashMap, la definiremos tq Username -> Valor
    pub usuarios: HashMap<String, (Transmisor, EstadoUsuario)>,

    //LLave2 del HashMap2, la definiremos tq Sala -> Valor
    pub salas: HashMap<String, Salas>,
}

impl EstadoServidor{
    
    pub fn nuevo() -> Self{
        EstadoServidor{
            usuarios: HashMap::new(),
            salas: HashMap::new(),
        }
    } 
}

//Aquí se ocupa el ARC para que pueda tener "multiple ownership" cosa que Rust no dejá 
//Mutex lo ocupamos para que no haya un choque entre los hilos de ejecución y siga siendo
//asincrono que es como Tokio trabaja :p
//De aquí leí lo de ARC https://medium.com/@Murtza/mastering-rust-arc-and-mutex-a-comprehensive-guide-to-safe-shared-state-in-concurrent-programming-1913cd17e08d
pub type EstadoCompartido = Arc<Mutex<EstadoServidor>>;