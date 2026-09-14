use std::collections::HashMap;
use serde_json::{Deserialize, Serialize};
use serde_json::Result;

//Utilizamos Enums para englobar todos los casos del protocolo 
//y no sea asqueriso y tener cada type escrito como r#type u algo así 
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MensajesDeEntrada{

    IDENTIFY{
        username: String
    },

    RESPONSE{
        operation: String,
        result: String,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        extra: Option<String>,
    },

    NEW_USER{
        username: String,
    },

    //Inicia la parte del estado del usuario
    STATUS{
        status: String,
    }
    
    NEW_STATUS{
        username: String,
        status: String,
    },


    //Solicita la lista de usuarios en el chat, ocupar un hash map
    //y un rojinegro, ya veo en que lo agrego este último
    USERS,

    //La respuesta del server al pedir la lista de usuarios
    USER_LIST{
        users: HashMap<String, String>,
    }
    
    TEXT{
        username: String,
        text: String,
    },

    TEXT_FROM{
        username: String,
        text: String,
    },

    PUBLIC_TEXT{
        text: String,
    },
    
    PUBLIC_TEXT_FROM{
        username: String,
        text: String,
    },

    //Parte del protocolo para las salas
    NEW_ROOM{
        roomname: String,
    },

    INVITE{
        roomname: String,
        usernames: Vect<String>,
    },

    INVITATION{
        username: String,
        roomname: String,
    },

    JOIN_ROOM{
        roomname: String,
    },

    JOINED_ROOM{
        roomname: String, 
        username: String,
    },

    //Ususarios de la salas
    ROOM_USER_LIST{
        roomname: String,
        users: HashMap<String, String>,
    },

    //Texto dentro de una sala
    
    ROOM_TEXT{
        roomname: String,
        text: String,
    },

    ROOM_TEXT_FROM{
        roomname: String,
        username: String,
        text: String,
    },

    //Salir de la sala
    LEAVE_ROOM{
        roomname: String,
    },

    LEFT_ROOM{
        roomname: String,
        username+: String,
    },

    //Se desconecta el usuario que noob 
    DISCONNECT,

    DISCONNECTED{
        username: String,
    },

}
