import 'protocolo.dart';

abstract class ClienteComando {
  Map<String, dynamic> toJson();
}

//Para el "Identify" del protocolo, esto debe de mandar el cliente
class IdentifyComando implements ClienteComando {
  final String username;

  IdentifyComando({required this.username}){
    if (username.length > 8) {
      throw new ArgumentError("El nombre no debe de exceder los 8 caractéres");
    }
  }

  @override
  Map<String, dynamic> toJson() => {
    'type': 'IDENTIFY',
    'username': username,
  };
}

//Para Status
class StatusComando implements ClienteComando {
  final EstadoUsuario estado;

  StatusComando({required this.estado});

  @override
  Map<String, dynamic> toJson() => {
    'type': 'STATUS',
    'status': estado.valorJson,
  };
}

//Para la lista de usuarios
class UsersComando implements ClienteComando {
  @override
  Map<String, dynamic> toJson() => {
    'type': 'USERS'
  };
}

//Para el Texto
class TextComando implements ClienteComando {
  final String username;
  final String texto;

  TextComando({required this.username, required this.texto});

  @override
  Map<String, dynamic> toJson() => {
    'type': 'TEXT',
    'username': username,
    'text': texto,  
  };
}

//Para el tecto público
class PublicTextComando implements ClienteComando {
  final String texto;
  
  PublicTextComando({required this.texto});

  @override
  Map<String, dynamic> toJson() => {
    'type': 'PUBLIC_TEXT',
    'text': texto,
  };
}

//Para el caso de un cuarto nuevo que se cree
class NewRoomComando implements ClienteComando {
  final String roomname;

  NewRoomComando({required this.roomname}){
    if(roomname.length > 16) {
      throw ArgumentError('El nombre de la sala no puede exceder 16 caracteres');
    }
  }

  @override
  Map<String, dynamic> toJson() => {
    'type': 'NEW_ROOM',
    'roomname': roomname,
  };
}

//Para el comando de INVITE
class InviteComando implements ClienteComando {
  final String roomname;
  final List<String> usersnames;

  InviteComando({required this.roomname, required this.usersnames});

  @override
  Map<String, dynamic> toJson() => {
    'type': 'INVITE',
    'roomname': roomname,
    'usernames': usersnames,
  };
}

//para el comando de JOINED ROOM 
class JoinRoomComando implements ClienteComando {
  final String roomname;

  JoinRoomComando({required this.roomname});

  @override
  Map<String, dynamic> toJson() => {
    'type': 'JOIN_ROOM',
    'roomname': roomname,
  };
}

//Para el caso de la lista de usuarios en una sala
class RoomUsersComando implements ClienteComando {
  final String roomname;

  RoomUsersComando({required this.roomname});

  @override
  Map<String, dynamic> toJson() => {
    'type' : 'Room_Users',
    'roomname': roomname,
  };
}

//Para el room text
class RoomTextComando implements ClienteComando {
  final String roomname;
  final String text;

  RoomTextComando({required this.roomname, required this.text});

  @override
  Map<String, dynamic> toJson() => {
    'type': 'ROOM_TEXT',
    'roomname': roomname,
    'text': text,
  };
}

//Para cuando abandona un cuarto
class LeaveRoomComando implements ClienteComando {
  final String roomname;

  LeaveRoomComando({required this.roomname});
  
  @override
  Map<String, dynamic> toJson() => {
    'type': 'LEAVE_ROOM',
    'roomname': roomname,
  };
}

//Para cuando se va al lobby
class DisconnectComando implements ClienteComando {
  @override
  Map<String, dynamic> toJson() => {
    'type': 'DISCONNECTED',
  };
}



