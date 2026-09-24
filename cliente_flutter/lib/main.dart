import 'dart:async';
import 'package:flutter/foundation.dart';
import 'controlador/controlador.dart';

void main() async{
  final controlador = Controlador();

  controlador.addListener(() {
    print('\n[UI ACTUALIZADA - notifyListeners disparado]');
    print(' - Conectado: ${controlador.estaConectado}');
    print(' - Usuarios Globales: ${controlador.usuariosConectados.keys.join(', ')}');
    print(' - Salas: ${controlador.usuariosPorSala.keys.join(', ')}');
    print(' - Mensajes Públicos: ${controlador.historialPublicoChat.length}');
  });

  try {
    print('Pruebas controladorcin');

    print('\n1.Conectando como John Doe');
    await controlador.conectarEIdentificar('127.0.0.1', 1234, 'John Doe');
    await Future.delayed(const Duration(seconds: 1));

    print('\n2.Mensaje de Prueba');
    controlador.mandarTextoPublico('Webos');
    await Future.delayed(const Duration(seconds: 1));

    print('\n3.Creando una sala');
    controlador.crearSala('Sala chida');
    await Future.delayed(const Duration(seconds: 1));

    print('\n4.Uniendose a la sala chida');
    controlador.entrarSala('Sala chida');
    await Future.delayed(const Duration(seconds: 1));

    print('\n5.Enviar msj la sala chida');
    controlador.mandarTxtSala('Sala chida', 'Mensaje épico');
    await Future.delayed(const Duration(seconds: 1));

    print('\n6. Abandona la sala');
    controlador.salirSala('Sala chida');
    await Future.delayed(const Duration(seconds: 1));

    print('\n7.Desconectando al cliente');
    controlador.desconectar();

    print('\n Si funcionó esto lol');

  }catch(e) {
    print('Ocurrio un error: $e');
  }finally {
    controlador.desconectar();
  }


}