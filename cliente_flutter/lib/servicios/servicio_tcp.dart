import 'dart:async';
import 'dart:io';
import 'dart:convert';

import '../modelos/cliente_comandos.dart';
import '../modelos/mensajes_server.dart';

class TcpServicio {
  Socket? _socket;

  final _mensajeControlador = StreamController<MensajeServer>.broadcast();

  Stream<MensajeServer> get mensaje => _mensajeControlador.stream;

  Future<void> connect(String host, int puerto) async {
    try{
      _socket = await Socket.connect(host, puerto);

      //Inicia el parseo en UTF8 y el pipeline 
      _socket! 
        .cast<List<int>>()
        .transform(utf8.decoder)
        .transform(const LineSplitter())
        .listen(
          _datosRecibidos,
          onError: (e) {
            _mensajeControlador.addError('Error en la conexicón del Socket: $e');
          },
          onDone: () {
            desconectar();
          },
        );
    }catch(e) {
      throw Exception('No se puede establecer la conexión TCP');
    }
  }

  void _datosRecibidos(String linea) {
    //Se agrega la parte del protocolo que cada línea del Json debe de tener el \n
    final lineaProcesada = linea.trim();

    if(lineaProcesada.isEmpty){
      return;
    };

    try{
      //Se hace el mapeo a Json 
      final dynamic decoded = jsonDecode(lineaProcesada);
            if (decoded is Map) {
              final Map<String, dynamic> jsonMap = Map<String, dynamic>.from(decoded);
              final serverMessage = MensajeServer.fromJson(jsonMap);
              _mensajeControlador.add(serverMessage);
            }

    }catch(e) {
      _mensajeControlador.addError('Error en la decodificación');
    }
  }

  void mandarComando(ClienteComando comando){
    if(_socket == null){
      throw StateError('No hya conexión con el server');
    }

    final jsonString = jsonEncode(comando.toJson());
    final mensajeSaltoLinea = '$jsonString\n';

    _socket!.write(mensajeSaltoLinea);
  }
  
  Future<void> desconectar() async {
    await _socket?.close();

    _socket?.destroy();
    _socket = null;
  }

  void desechar(){
    desconectar();
    _mensajeControlador.close();
  }


  
}