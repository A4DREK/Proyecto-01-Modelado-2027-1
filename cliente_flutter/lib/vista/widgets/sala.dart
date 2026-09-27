import 'package:flutter/material.dart';
import '../../controlador/controlador.dart';

Future<String?> mostrarDialogosSala(BuildContext contexto, Controlador controlador) async {
  final control = TextEditingController();

  return showDialog<String>(
    context: contexto,
    builder: (dialogoContexto) {
      return AlertDialog(
        title: const Text('Unirse o crear nueva sala'),
        content: TextField(
          controller: control,
          autofocus: true,
          decoration: const InputDecoration(
            hintText: 'Nombre de la sala ',
            border: OutlineInputBorder(),
          ),
          onSubmitted: (nombreSala) {
            _procesarEntrada(dialogoContexto, controlador, nombreSala);
          },
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialogoContexto, null),
            child: const Text('Cancelar'),
          ),
          ElevatedButton(
            onPressed: () {
              _procesarEntrada(dialogoContexto, controlador, control.text);
            },
            child: const Text('Entrar'),
          ),
        ],
      );
    },
  );
}

void _procesarEntrada(BuildContext contexto, Controlador controlador, String nombre) {
  final sala = nombre.trim();

  if(sala.isNotEmpty) {
    controlador.entrarSala(sala);
    Navigator.pop(contexto, sala);
  }
}