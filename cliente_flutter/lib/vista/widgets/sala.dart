import 'package:flutter/material.dart';

import '../../controlador/controlador.dart';

enum AccionSala { crear, unirse }

class ResultadoSala {
  final AccionSala accion;
  final String nombre;
  ResultadoSala(this.accion, this.nombre);
}

Future<ResultadoSala?> mostrarDialogosSala(
  BuildContext contexto,
  Controlador controlador,
) async {
  final control = TextEditingController();

  return showDialog<ResultadoSala>(
    context: contexto,
    builder: (dialogoContexto) {
      return AlertDialog(
        title: const Text('Salas Chat'),
        content: TextField(
          controller: control,
          autofocus: true,
          decoration: const InputDecoration(
            hintText: 'Nombre de la sala ',
            border: OutlineInputBorder(),
          ),
          onSubmitted: (nombreSala) {
            final sala = nombreSala.trim();
            if (sala.isNotEmpty) {
              Navigator.pop(
                dialogoContexto,
                ResultadoSala(AccionSala.crear, sala),
              );
            }
          },
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialogoContexto, null),
            child: const Text('Cancelar'),
          ),
          OutlinedButton(
            onPressed: () {
              final sala = control.text.trim();
              if (sala.isNotEmpty) {
                Navigator.pop(
                  dialogoContexto,
                  ResultadoSala(AccionSala.unirse, sala),
                );
              }
            },
            child: const Text('Unirse'),
          ),
        ],
      );
    },
  );
}
