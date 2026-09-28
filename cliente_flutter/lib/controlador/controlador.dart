import 'dart:async';
import 'package:flutter/foundation.dart';

import '../modelos/cliente_comandos.dart';
import '../modelos/mensajes_server.dart';
import '../modelos/protocolo.dart';
import '../servicios/servicio_tcp.dart';

class Controlador extends ChangeNotifier {

  final TcpServicio _tcpServicio = TcpServicio();
  StreamSubscription<MensajeServer>? _mensajesSubscripcion;

  //Estados del chat  en el UI
  bool estaConectado = false;

  //Parte del usuario
  String? miUsuario;
  EstadoUsuario miEstado = EstadoUsuario.active;

  //Lista de los usuarios conectados
  Map<String, EstadoUsuario> usuariosConectados = {};

  //Historial del chat
  final List<MensajeServer> historialPublicoChat = []; 

  //Chats privados 
  final Map<String, List<TextFromMsj>> chatPrivados = {};

  //Salas unidas e integrantes
  final Map<String, List<RoomTextFromMsj>> historialSalas = {};

  //Usuarios de una sala
  final Map<String, Map<String, EstadoUsuario>> usuariosPorSala = {};

  //Invitaciones
  final List<InvitationMsj> invitacionesPendientes = [];

  final _errorControl = StreamController<String>.broadcast();
  Stream<String> get streamErrores => _errorControl.stream;

  ResponseMsj? ultimaRespuesta; 

  //En esta parte inicia el server y las conexiones

  Future<void> conectarEIdentificar(String host, int puerto, String username) async{
    try{

      //En esta parte se espera una conección de un hilo, para pasar el bool a true y el usuario con el username
      await _tcpServicio.connect(host, puerto);
      estaConectado = true;
      miUsuario = username;
      notifyListeners(); //El aviso a la UI de la conexion

      _mensajesSubscripcion = _tcpServicio.mensaje.listen(
        _manejarMsjEntrante,
        onError: (e) {
          estaConectado = false;
          notifyListeners();
        }
      );

      _tcpServicio.mandarComando(IdentifyComando(username: username));
      _tcpServicio.mandarComando(UsersComando());

    }catch(e) {
      estaConectado = false;
      notifyListeners();
      rethrow; // Para mostrar el error en la UI
    }
  }

  void _manejarMsjEntrante(MensajeServer msj) {
    switch (msj) {

      ///Caso donde se responden msj
      case ResponseMsj respuesta :
        ultimaRespuesta = respuesta;
        if(respuesta.operation == 'IDENTIFY' && respuesta.result == 'SUCCESS') {
          estaConectado = true;
        }
        if((respuesta.operation == 'NEW_ROOM' || respuesta.operation == 'JOIN_ROOM') && respuesta.extra != null ){
          _inicializarSala(respuesta.extra!);
        }else {
          final msjErr = _mapearMsjError(respuesta);
          _errorControl.add(msjErr);
        }
        notifyListeners();
        break;

      //Caso donde hay un nuevo usuario
      case NewUserMsj nuevoUsuario:
        usuariosConectados[nuevoUsuario.username] = EstadoUsuario.active;
        notifyListeners();
        break;

      //Caso del nuevo status
      case NewStatusMsj nuevoStatus : 
        usuariosConectados[nuevoStatus.username] = nuevoStatus.status;

        if(nuevoStatus.username == miUsuario) {
          miEstado = nuevoStatus.status;
        }

        for(final miembrosSala in usuariosPorSala.values) {
          if(miembrosSala.containsKey(nuevoStatus.username)) {
            miembrosSala[nuevoStatus.username] = nuevoStatus.status;
          }
        }

        notifyListeners();
        break;

      case UserListMsj listaUsuario :
        usuariosConectados = Map.from(listaUsuario.users);
        notifyListeners();
        break;
      
      case TextFromMsj textoPriv :

        chatPrivados
          .putIfAbsent(textoPriv.username, () => [] )
          .add(textoPriv);
        notifyListeners();
        break;

      case PublicTextFromMsj msjPub : 
        historialPublicoChat.add(msjPub);
        notifyListeners();
        break;
      
      case InvitationMsj invitacion: 
        if(!invitacionesPendientes.any((inv) => inv.roomname == invitacion.roomname)) {
          invitacionesPendientes.add(invitacion);
          _tcpServicio.mandarComando(RoomUsersComando(roomname: invitacion.roomname));
        }
        notifyListeners();
        break;

      case RoomUserListMsj listaUsuariosSala:
        usuariosPorSala[listaUsuariosSala.roomname] = Map.from(listaUsuariosSala.users);
        notifyListeners();
        break;

      case RoomTextFromMsj textoSala: 
        historialSalas
          .putIfAbsent(textoSala.roomname, () => [])
          .add(textoSala);
        notifyListeners();
        break;

      case LeftRoomMsj salioSala :
        if(salioSala.username == miUsuario) {
          usuariosPorSala.remove(salioSala.roomname);
          historialSalas.remove(salioSala.roomname);
        }else {
          usuariosPorSala[salioSala.roomname]?.remove(salioSala.username);

          historialSalas[salioSala.roomname]?.add(
            RoomTextFromMsj(
              roomname: salioSala.roomname,
              username: 'Sistema',
              text: '${salioSala.username} ha salido de la sala',
            ),
          );
        }
        notifyListeners();
        break;

      case DisconnectedMsj msjDisco :
        usuariosConectados.remove(msjDisco.username);
        for(final miembrosSala in usuariosPorSala.values) {
          miembrosSala.remove(msjDisco.username);
        }
        notifyListeners();
        break;
      
      case JoinedRoomMsj() :
        usuariosPorSala.putIfAbsent(msj.roomname, () => {});

        usuariosPorSala[msj.roomname]?[msj.username] = EstadoUsuario.active;

        historialSalas.putIfAbsent(msj.roomname, () => []).add(
          RoomTextFromMsj(
            roomname: msj.roomname,
            username: 'Sistema',
            text: '${msj.username} se ha unido a la sala'
          ),
        );

        notifyListeners();
        break;
      default:
        break; //Falta implementar el resto de los msj 
    }
  }

  //Estos métodos son para el envio de las acciones a la red, todo lo asíncrono y así :p
  void cambiarEstado(EstadoUsuario nuevoEstado) {
    miEstado = nuevoEstado;

    if(miUsuario != null){
       usuariosConectados[miUsuario!] = nuevoEstado;
    }

    notifyListeners();

    _tcpServicio.mandarComando(StatusComando(estado: nuevoEstado));
  }

  void mandarTextoPublico(String texto) {
    _tcpServicio.mandarComando(PublicTextComando(texto: texto));
    if(miUsuario != null) {
      historialPublicoChat.add(
        PublicTextFromMsj(username: miUsuario!, text: texto),
      );
      notifyListeners();
    }
  }

  void mandarTxtPriv(String usuarioDestino, String texto) {
    _tcpServicio.mandarComando(TextComando(username: usuarioDestino, texto: texto));

    chatPrivados.putIfAbsent(usuarioDestino, () => []).add(
      TextFromMsj(username: miUsuario ?? 'Yo', text: texto),
    );

    notifyListeners();
  }

  void crearSala(String roomname) {
    _inicializarYUnirser(roomname);
    _tcpServicio.mandarComando(NewRoomComando(roomname: roomname));
  }

  void invitarSala(String roomname, List<String> usernames) {
    _inicializarYUnirser(roomname);
    _tcpServicio.mandarComando(InviteComando(roomname: roomname, usersnames: usernames));
  }

  void entrarSala(String roomname) {
    _inicializarYUnirser(roomname);
    _tcpServicio.mandarComando(JoinRoomComando(roomname: roomname));
  }

  void mandarTxtSala(String roomname, String texto) {
    _tcpServicio.mandarComando(RoomTextComando(roomname: roomname, texto: texto));
    if(miUsuario != null) {
      historialSalas.putIfAbsent(roomname, () => []).add(
        RoomTextFromMsj(
          roomname: roomname,
          username: miUsuario!,
          text: texto,
        ),
      );
      notifyListeners();
    }
  }

  void salirSala(String roomname) {
    _tcpServicio.mandarComando(LeaveRoomComando(roomname: roomname));
    usuariosPorSala.remove(roomname);
    historialSalas.remove(roomname);
    notifyListeners();
  }

  void desconectar() {
    if(estaConectado) {
      _tcpServicio.mandarComando(DisconnectComando());
    }
    _tcpServicio.desconectar();
    estaConectado = false;
    notifyListeners();
  }

  //Método aux para las salas
  void _inicializarYUnirser(String roomname){
    usuariosPorSala.putIfAbsent(roomname, () => {});
    historialSalas.putIfAbsent(roomname, () => []);

    if(miUsuario != null) {
      usuariosPorSala[roomname]?[miUsuario!] = miEstado;
    }

    _tcpServicio.mandarComando(RoomUsersComando(roomname: roomname));
    notifyListeners();
  }

  //Método aux para el mapeo del error
  String _mapearMsjError(ResponseMsj respuesta){
    final codigo = respuesta.result.toUpperCase().trim();
    final detalleExtra = respuesta.extra != null ? '(${respuesta.extra})' : '' ;

    return switch (codigo) {
      'ROOM_ALREADY_EXISTS' => 'La sala ya existe. Elige otro nombre.',
      'NO_SUCH_USER'        => 'El usuario no existe o se desconectó$detalleExtra.',
      'NO_SUCH_ROOM'        => 'La sala no existe$detalleExtra.',
      'NOT_JOINED'          => 'No formas parte de esta sala.',
      'NOT_INVITED'         => 'No has sido invitado a esta sala.',
      'USER_ALREADY_EXISTS' => 'El nombre de usuario ya está ocupado.',
      'NOT_IDENTIFIED'      => 'Sesión no válida o no identificada.',
      'INVALID'             => 'Comando o formato de datos inválido.',
      _                     => 'Operación fallida en ${respuesta.operation}: $codigo',
    };
  }

  void _inicializarSala(String roomname) {
    usuariosPorSala.putIfAbsent(roomname, () => {});
    historialSalas.putIfAbsent(roomname, () => []);

    if (miUsuario != null) {
      usuariosPorSala[roomname]?[miUsuario!] = miEstado;
    }

    _tcpServicio.mandarComando(RoomUsersComando(roomname: roomname));
  }

  @override
  void dispose() {
    _mensajesSubscripcion?.cancel();
    _tcpServicio.desechar();
    _errorControl.close();
    super.dispose();
  }
}
