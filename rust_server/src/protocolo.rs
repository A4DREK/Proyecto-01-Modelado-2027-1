use std::collections::HashMap;
use serde::{Deserialize, Serialize};

//Utilizamos Enums para englobar todos los casos del protocolo 
//y no sea asqueroso y tener cada type escrito como r#type u algo así 


//Esta parte es para el etado del usuario del Status.
#[derive(Serialize, Deserialize, Debug,PartialEq, Clone)]
pub enum EstadoUsuario{
    ACTIVE,
    AWAY,
    BUSY,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[allow(non_camel_case_types)]
pub enum MensajesDeEntrada{
    
    IDENTIFY{
        username: String,
    },

    STATUS{
        status: EstadoUsuario, 
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
        usernames: Vec<String>,
    },

    JOIN_ROOM{
        roomname: String,
    },

    ROOM_USERS{
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
//de los mensajes que se envian desde el server todo esto es 
//para la parte de TODOS los responses para no repetir varias
//veces lo mismo
//El tipo de operación que mandará el enum
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum Operacion{
    IDENTIFY,
    TEXT,
    NEW_ROOM,
    INVITE,
    JOIN_ROOM,
    ROOM_USERS,
    ROOM_TEXT,
    LEAVE_ROOM,
    INVALID,

}

//Enum para el resultado de la operación depende del caso
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum ResultadoOperacion{
    SUCCESS,
    USER_ALREADY_EXISTS,
    INVALID_USERNAME,
    NO_SUCH_USER,
    NOT_IDENTIFIED,
    INVALID,
    

    NEW_ROOM,
    ROOM_ALREADY_EXISTS,
    NO_SUCH_ROOM,
    NOT_INVITED,
    NOT_JOINED,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[allow(non_camel_case_types)]
pub enum MensajesDeSalida{
    RESPONSE{
        operation: Operacion,
        resultado: ResultadoOperacion, 

        #[serde(skip_serializing_if = "Option::is_none")]
        extra: Option<String>,
    },

    NEW_USER{
        username: String,
    },

    NEW_STATUS{
        username: String,
        status: EstadoUsuario,
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
        users: HashMap<String, String>,
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

#[cfg(test)]
mod test{
    use super::*;
    use serde_json::{from_str, to_string};

    //Mensajes de entrada del JSON 
    #[test]
    fn test_identify(){
        let json_original = r#"{"type":"IDENTIFY","username":"Adam"}"#;
        let resutlado_json = from_str(json_original).unwrap();

        match resutlado_json{
            MensajesDeEntrada::IDENTIFY { username } => {
                assert_eq!(username, "Adam");
            }
            _ => panic!("Error, no hubo parseo de IDENTIFY"),
        }
    }

    #[test]
    fn test_status(){

        let json_original = r#"{"type":"STATUS","status":"AWAY"}"#;
        let resutlado_json: MensajesDeEntrada = from_str(json_original).unwrap();

        match resutlado_json {
            MensajesDeEntrada::STATUS { status } => {
                assert_eq!(status, EstadoUsuario::AWAY);
            }
            _ => panic!("Espera de variable AWAY"),
        }
    }

    #[test]
    fn test_public_text(){
        let json_original = r#"{"type":"PUBLIC_TEXT","text":"¡Puro Toros Neza papá!"}"#;
        let resultado_json = from_str(json_original).unwrap();

        match resultado_json{
            MensajesDeEntrada::PUBLIC_TEXT { text } => {
                assert_eq!(text, "¡Puro Toros Neza papá!");
            }
            _ => panic!("Esperaba Variante PUBLIC_TEXT"),
        }
    }

    #[test]
    fn test_private_test(){
        let json_original = r#"{"type":"TEXT","username":"Paco","text":"Es un msj secreto"}"#;
        let resultado_json = from_str(json_original).unwrap();

        match resultado_json {
            MensajesDeEntrada::TEXT { username, text } => {
                assert_eq!(username, "Paco");
                assert_eq!(text, "Es un msj secreto");
            }
            _ => panic!("Esperaba Variante TEXT"),
        }
    }

    #[test]
    fn test_room_users(){
        let json_original = r#"{"type":"ROOM_USERS","roomname":"Sala 1"}"#;
        let resultado_json: MensajesDeEntrada = from_str(json_original).unwrap();

        match resultado_json {
            MensajesDeEntrada::ROOM_USERS { roomname } => {
                assert_eq!(roomname, "Sala 1");
            }
            _ => panic!("Espera de variante ROOM_USERS"),
        }
    }

    

    #[test]
    fn test_json_sin_parametros(){
        //Para los identufy de ROOM, USERS, DISCONNECT
        let json_users = r#"{"type":"USERS"}"#;
        let json_disconnect = r#"{"type":"DISCONNECT"}"#;

        let msg_users: MensajesDeEntrada = from_str(json_users).unwrap();
        let msg_disconnect: MensajesDeEntrada = from_str(json_disconnect).unwrap();

        assert!(matches!(msg_users, MensajesDeEntrada::USERS));
        assert!(matches!(msg_disconnect, MensajesDeEntrada::DISCONNECT));
    }

    #[test]
    fn test_json_invalido(){
        let json_invalido = r#"{"usuario":"Max Versttapen"}"#;
        let resultado_json = serde_json::from_str::<MensajesDeEntrada>(json_invalido);

        assert!(resultado_json.is_err(), "Debe de fallar en el campo type de JSON");
    }

    #[test]
    fn test_json_desconocido(){
        let json_desconocido = r#"{"type":"COMANDO_FANTASMA"}"#;
        let resultado_json = from_str::<MensajesDeEntrada>(json_desconocido);

        assert!(resultado_json.is_err(), "Un type desconocido, el Enum debe de fallar");
    }

    //Pruebas para los mensajes de salida 
    #[test]
    fn test_salida_respuesta_extra(){
        let respuesta = MensajesDeSalida::RESPONSE { 
            operation: Operacion::IDENTIFY, 
            resultado: ResultadoOperacion::SUCCESS,
            extra: Some("Bienvenido al server".to_string())
        };

        let resultado_json = to_string(&respuesta).unwrap();

        assert!(resultado_json.contains(r#""type":"RESPONSE""#));
        assert!(resultado_json.contains(r#""operation":"IDENTIFY""#));
        assert!(resultado_json.contains(r#""resultado":"SUCCESS""#));
        assert!(resultado_json.contains(r#""extra":"Bienvenido al server""#));

    }

    #[test]
    fn test_salida_omite_campo(){
        let respuesta = MensajesDeSalida::RESPONSE {
            operation: Operacion::IDENTIFY,
            resultado: ResultadoOperacion::SUCCESS,
            extra: None, 
        };

        let resultado_json = to_string(&respuesta).unwrap();

        assert!(!resultado_json.contains("extra"),"El campo extra no debe de aparecer si es None");
    }

    #[test]
    fn test_salida_lista_usuarios(){

        let mut usuarios = HashMap::new();
        usuarios.insert("Lewis Hamilton".to_string(), "ACTIVE".to_string());
        usuarios.insert("Banito Martines".to_string(), "AWAY".to_string());

        let respuesta_json = MensajesDeSalida::USER_LIST { users: usuarios };
        let resultado_json = to_string(&respuesta_json).unwrap();

        assert!(resultado_json.contains(r#""type":"USER_LIST""#));
        assert!(resultado_json.contains(r#""Lewis Hamilton":"ACTIVE""#));
    }
}