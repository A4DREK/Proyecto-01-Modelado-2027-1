import 'dart:io';
import 'package:flutter/material.dart';
import 'controlador/controlador.dart';
import 'vista/pantalla_principal.dart';

void main(List<String> args) async {
  print('\n Chat MyP');

//Validación de los argumentos escritos en la terminal
  if(args.length < 3) {
    print('Faltan arguemntos en la terminal para inicializar al cliente');
    print('Forma de escribir el comando: ');
    print(' <comando> <IP> <Puerto> <Usuario>');
    print('Ejemplo:');
    print(' flutter run -d macos -a 127.0.0.1 -a 1234 -a Aly\n');
    exit(1);
  }

  final ip = args[0];
  final puerto = int.tryParse(args[1]) ?? 1234;
  final username = args.sublist(2).join(' ');

  final controlador = Controlador();

  try {
    
    await controlador.conectarEIdentificar(ip, puerto, username);
     
  }catch(e) {
    print('error en la conxión');
    exit(1);
  }

  runApp(ChatApp(controlador: controlador));


}