enum EstadoUsuario {
  active('ACTIVE'),
  away('AWAY'),
  busy('BUSY');

  final String valorJson;
  const EstadoUsuario(this.valorJson);

  factory EstadoUsuario.fromString(String statusStr) {
    return EstadoUsuario.values.firstWhere(
      (e) => e.valorJson == statusStr,
      orElse: () => EstadoUsuario.active,
    );
  }
}

enum MensajesServidorType {
  response('RESPONSE'),
  newUser('NEW_USER'),
  newStatus('NEW_STATUS'),
  userList('USER_LIST'),
  textFrom('TEXT_FROM'),
  publicTextFrom('PUBLIC_TEXT_FROM'),
  invitation('INVITATION'),
  joinedRoom('JOINED_ROOM'),
  roomUserList('ROOM_USER_LIST'),
  roomTextFrom('ROOM_TEXT_FROM'),
  leftRoom('LEFT_ROOM'),
  disconnected('DISCONNECTED'),
  unknown('UNKNOWN');

  final String valorJson;
  const MensajesServidorType(this.valorJson);

  factory MensajesServidorType.fromString(String typeStr) {
    return MensajesServidorType.values.firstWhere(
      (e) => e.valorJson == typeStr,
      orElse: () => MensajesServidorType.unknown,
    );
  }}