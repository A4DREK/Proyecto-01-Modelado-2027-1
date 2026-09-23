import 'protocolo.dart';

sealed class MensajeServer {
  final MensajesServidorType type;
  const MensajeServer(this.type);

  factory MensajeServer.fromJson(Map<String, dynamic> json) {
    final typeStr = json['type']?.toString();

    if(typeStr == null) {
      return UnknownMsj(json);
    }

    final msjType = MensajesServidorType.fromString(typeStr);

    switch (msjType) {
      case MensajesServidorType.response:
        return ResponseMsj.fromJson(json);
      case MensajesServidorType.newUser:
        return NewUserMsj.fromJson(json);
      case MensajesServidorType.newStatus:
        return NewStatusMsj.fromJson(json);
      case MensajesServidorType.userList:
        return UserListMsj.fromJson(json);
      case MensajesServidorType.textFrom:
        return TextFromMsj.fromJson(json);
      case MensajesServidorType.publicTextFrom:
        return PublicTextFromMsj.fromJson(json);
      case MensajesServidorType.invitation:
        return InvitationMsj.fromJson(json);
      case MensajesServidorType.joinedRoom:
        return JoinedRoomMsj.fromJson(json);
      case MensajesServidorType.roomUserList:
        return RoomUserListMsj.fromJson(json);
      case MensajesServidorType.roomTextFrom:
        return RoomTextFromMsj.fromJson(json);
      case MensajesServidorType.leftRoom:
        return LeftRoomMsj.fromJson(json);
      case MensajesServidorType.disconnected:
        return DisconnectedMsj.fromJson(json);
      case MensajesServidorType.unknown:
        return UnknownMsj(json);
    }
  }
}

//EN ESTA PARTE SE INICIARÁ A HACER LA IMPLEMENTACIÓN DE LOS MSJ 

//Para los casos donde el server debe de devolver RESPONSE, que se decodifique el JSON 
class ResponseMsj extends MensajeServer {
  final String operation; 
  final String result;
  final String? extra;

  //Constructor del responseMsj del protocolo para los RESPONSE
  ResponseMsj({
    required this.operation,
    required this.result,
    this.extra,
  }): super(MensajesServidorType.response);

  factory ResponseMsj.fromJson(Map<String, dynamic> json) {
    return ResponseMsj(
      operation: json['operation']?.toString() ?? '',
      result: json['result']?.toString() ?? '',
      extra: json['extra']?.toString() ?? '',
    );

  }
}

//Para el caso de new user
class NewUserMsj extends MensajeServer {
  final String username;

  NewUserMsj({
    required this.username
  }): super(MensajesServidorType.newUser);

  factory NewUserMsj.fromJson(Map<String, dynamic> json) {
    return NewUserMsj(
      username: json['username']?.toString() ?? '',
    );
  }
}

//Para el caso de New Status 
class NewStatusMsj extends MensajeServer {
  final String username;
  final EstadoUsuario status;

  NewStatusMsj({
    required this.username,
    required this .status,
  }): super(MensajesServidorType.newStatus);

  factory NewStatusMsj.fromJson(Map<String, dynamic> json) {
    return NewStatusMsj(
      username: json['username']?.toString() ?? '',
      status: EstadoUsuario.fromString(json['status']?.toString() ?? ''),
    );
  }
}

//Para la User List 
class UserListMsj extends MensajeServer {
  final Map<String, EstadoUsuario> users;

  UserListMsj({
    required this.users,
  }): super(MensajesServidorType.userList);

  factory UserListMsj.fromJson(Map<String, dynamic> json) {
    final usuariosOG = json['users'] as Map<String, dynamic>? ?? {};
    final usuariosMod = usuariosOG.map(
      (llave, valor) => MapEntry(llave, EstadoUsuario.fromString(valor.toString())),
    );

    return UserListMsj(users: usuariosMod);
  }
}

//Para el text from 
class TextFromMsj extends MensajeServer {
  final String username;
  final String text;

  TextFromMsj({
    required this.username,
    required this.text,
  }): super(MensajesServidorType.textFrom);

  factory TextFromMsj.fromJson(Map<String, dynamic> json){
    return TextFromMsj(
      username: json['username']?.toString() ?? '',
      text: json['text']?.toString() ?? '',
    );
  }
}

//Para el public text
class PublicTextFromMsj extends MensajeServer {
  final String username;
  final String text;

  PublicTextFromMsj({
    required this.username, 
    required this.text
    }): super(MensajesServidorType.publicTextFrom);

  factory PublicTextFromMsj.fromJson(Map<String, dynamic> json) {
    return PublicTextFromMsj(
      username: json['username']?.toString() ?? '',
      text: json['text']?.toString() ?? '',
    );
  }
}

//para la invitacion
class InvitationMsj extends MensajeServer {
  final String username;
  final String roomname;

  InvitationMsj({
    required this.username,
    required this.roomname,
  }): super(MensajesServidorType.invitation);

  factory InvitationMsj.fromJson(Map<String, dynamic> json) {
    return InvitationMsj(
      username: json['username']?.toString() ?? '',
      roomname: json['roomname']?.toString() ?? '',
    );
  }
}

//Para el caso de JoinedRoom
class JoinedRoomMsj extends MensajeServer {
  final String roomname;
  final String username;

  JoinedRoomMsj({
    required this.roomname,
    required this.username
    }): super(MensajesServidorType.joinedRoom);

  factory JoinedRoomMsj.fromJson(Map<String, dynamic> json) {
    return JoinedRoomMsj(
      roomname: json['roomname']?.toString() ?? '',
      username: json['username']?.toString() ?? '',
    );
  }
}

//Para la lista de usuarios en la sala
class RoomUserListMsj extends MensajeServer {
  final String roomname;
  final Map<String, EstadoUsuario> users;

  RoomUserListMsj({
    required this.roomname,
    required this.users
    }): super(MensajesServidorType.roomUserList);

  factory RoomUserListMsj.fromJson(Map<String, dynamic> json) {
    final usuariosOG = json['users'] as Map<String, dynamic>? ?? {};
    final usuariosMod = usuariosOG.map(
      (key, value) => MapEntry(key, EstadoUsuario.fromString(value as String)),
    );
    return RoomUserListMsj(
      roomname: json['roomname']?.toString() ?? '',
      users: usuariosMod,
    );
  }
}

//Para los msj de una sala
class RoomTextFromMsj extends MensajeServer {
  final String roomname;
  final String username;
  final String text;

  RoomTextFromMsj({
    required this.roomname,
    required this.username,
    required this.text,
  }): super(MensajesServidorType.roomTextFrom);

  factory RoomTextFromMsj.fromJson(Map<String, dynamic> json) {
    return RoomTextFromMsj(
      roomname: json['roomname']?.toString() ?? '',
      username: json['username']?.toString() ?? '',
      text: json['text']?.toString() ?? '',
    );
  }
}

//Para cuando un usuario se va de una sala
class LeftRoomMsj extends MensajeServer {
  final String roomname;
  final String username;

  LeftRoomMsj({
    required this.roomname,
    required this.username,
  }): super(MensajesServidorType.leftRoom);

  factory LeftRoomMsj.fromJson(Map<String, dynamic> json){
    return LeftRoomMsj(
      roomname: json['roomname']?.toString() ?? '',
      username: json['username']?.toString() ?? '',
    );
  }
}

//Para las desconexiones
class DisconnectedMsj extends MensajeServer {
  final String username;

  DisconnectedMsj({
    required this.username,
  }): super(MensajesServidorType.disconnected);

  factory DisconnectedMsj.fromJson(Map<String, dynamic> json) {
    return DisconnectedMsj(
      username: json['username']?.toString() ?? '',
    );
  }
}

//Por cualquier otra cosa que se llegará a presentar
class UnknownMsj extends MensajeServer {
  final Map<String, dynamic> json;

  UnknownMsj(
    this.json
  ) : super(MensajesServidorType.unknown);
}