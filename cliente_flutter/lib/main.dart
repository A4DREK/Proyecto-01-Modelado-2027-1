/*Este peuqeño cliente fue recuperado de: 
* https://medium.com/@arunthacharuthodi/tcp-socket-in-flutter-dart-io-library-cc50c65cb23c
* A partir de aquí se iniciará a hacer las modificaciones para que el cliente quede
* de acuerdo con las especificaciones del protocolo, estas estarán en el direcotrio de modelo y controlador
*/

/*
 * Hacemos la primera moficiacion para que acepte las partes de de servicio_tcp.dart  
 */
import 'dart:async';
import 'package:cliente_flutter/modelos/cliente_comandos.dart';
import 'package:cliente_flutter/modelos/mensajes_server.dart';
import 'package:cliente_flutter/servicios/servicio_tcp.dart';



void main () async {

  final tcpServicio = TcpServicio();

  tcpServicio.mensaje.listen(
    (MensajeServer mensaje) {
      print('\n--- Mensaje Recibido de Rust ---');
      print('Tipo: ${mensaje.type.valorJson}');

      switch (mensaje) {
        case ResponseMsj response:
          print('Operación: ${response.operation}');
          print('Resultado: ${response.result}');
          print('Extra: ${response.extra}');

        case NewUserMsj newUser:
          print('Nuevo usuario en el servidor: ${newUser.username}');
        
        case UserListMsj userList:
          print('Lista de usuarios recibida:');
          userList.users.forEach((username, status) {
            print('  - $username (${status.valorJson})');
          });
        case PublicTextFromMsj publicText:
          print('[Chat Público] ${publicText.username}: ${publicText.text}');
        
        default:
          print('Otro msj parseado bien :D');
      }
    },
    onError: (e) {
      print('[Error en Socket]: $e');
    },
    onDone: () {
      print('[CONEXIÓN CERRADA POR EL SERVIDOR]');
    }
  );

  try{
    print('Intentando conectar al servidor Rust en 127.0.0.1:1234...');
    await tcpServicio.connect('127.0.0.1', 1234);
    print('Conexión establecida.');

    //inicio de mandar todos los comandos askdsoifjqewodjew
    print('\n> Enviando IDENTIFY...');
    tcpServicio.mandarComando(IdentifyComando(username:'Kimberly'));
    await Future.delayed(const Duration(seconds: 1));

    print('\n> Enviando USERS...');
    tcpServicio.mandarComando(UsersComando());
    await Future.delayed(const Duration(seconds: 1));

    print('\n> Enviando PUBLIC_TEXT...');
    tcpServicio.mandarComando(PublicTextComando(texto: '¡Hola desde Flutter!'));
    await Future.delayed(const Duration(seconds: 2));

    print('\nFinalizando pruebas y cerrando socket...');
    await tcpServicio.desconectar();
  } catch (e) {
    print('Excepción atrapada: $e');
  } finally {
    tcpServicio.desechar();
  }
} 