enum JsonMsj {
  response('RESPONSE'),
  newUser('NEW_USER'),
  newStatus('NEW_STATUS'),
  userList('USER_LIST'),
  textFrom('TEXT_FROM'),
  publicTextFrom('PUBLIC_TEXT_FROM'),
  joinedRoom('JOINED_ROOM'),
  roomUserList('ROOM_USER_LIST'),
  roomTextFrom('ROOM_TEXT_FROM'),
  leftRoom('LEFT_ROOM'),
  disconnected('DISCONNECTED');

  final String valorJson;
  const JsonMsj(this.valorJson);

  factory JsonMsj.fromString(String jsonStr){
    return values.firstWhere(
      (e) => e.valorJson == jsonStr,
      orElse: () => throw FormatException('Comando Desconocido: $jsonStr'),
    );
  }
}

sealed class MensajesServer {
   
}