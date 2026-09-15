use std::collections::HashMap;
use serde_json::{Deserialize, Serialize};
use serde_json::Result;

//Utilizamos Enums para englobar todos los casos del protocolo 
//y no sea asqueriso y tener cada type escrito como r#type u algo así 
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MensajesDeEntrada{
    
    INDENTIFY{
        username: String,
    },

    Status{
        status: String,
    },
    
    USERS,

    TEXT{
        username: String,
        text: String,
    },

    PUBLIC_TEXT{
        text: String,
    },

    NEW_ROOM{
        roomname: String,
    },

    INVITE{
        roomname: String,
        usernames: Vec<Srting>,
    },

    JOIN_ROOM{
        roomname: String,
    },

    ROOM_TEXT{
        roomname: String, 
        text: String,
    },

    LEAVE_ROOM{
        roomname: String,
    },
    
    DISCONNECT,
}

//se agrego este nuevo enum para agrega la parte del protooclo 
//de los mensajes que se envian desde el server

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MensajesDeSalida{
    RESPONSE{
        operation: String,
        result: String, 

        #[serde(skip_serializing_if = "Option::in_none")]
        extra: Option<String>,
    },

    NEW_USER{
        username: String,
    },

    NEW_STATUS{
        username: String,
        status: String,
    },

    USER_LIST{
        users: HashMap<String, String>,
    },

    TEXT_FROM{
        username: String,
        text: String,
    },

    PUBLIC_TEXT_FROM{
        username: String,
        text: String,
    },

    INVITATION{
        username: String,
        roomname: String,
    },

    JOINED_ROOM{
        roomname: String,
        username: String,
    },

    ROOM_USER_LIST{
        roomname: String,
        users: HashMap<String>,
    },

    ROOM_TEXT_FROM{
        roomname: String,
        username: String,
        text: String,
    },

    LEFT_ROOM{
        roomname: String,
        username: String,
    },

    DISCONNECTED{
        username: String,
    },
}

