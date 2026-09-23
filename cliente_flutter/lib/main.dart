/*Este peuqeño cliente fue recuperado de: 
* https://medium.com/@arunthacharuthodi/tcp-socket-in-flutter-dart-io-library-cc50c65cb23c
* A partir de aquí se iniciará a hacer las modificaciones para que el cliente quede
* de acuerdo con las especificaciones del protocolo, estas estarán en el direcotrio de modelo y controlador
*/
import 'dart:io';

void main () async {
  try {
    final socket = await Socket.connect('127.0.0.1', 1234);
    print('Conected to server');

    // Listen for responses from the server
    socket.listen(
      (data) {
        print('Server Response: ${String.fromCharCodes(data).trim()}');
      },
      onDone: () {
        print('Connection closed by server');
        socket.destroy();
      },
      onError: (error) {
        print('Error: $error');
      },
    );

    final identification = '{"type":"IDENTIFY","username":"John Doe"}\n';
    socket.write(identification);

    // Close the connection after sending a message
    await Future.delayed(Duration(seconds: 2));
    await socket.close();
  }catch (e) {
    print('Unable to connect: $e');
  }
} 