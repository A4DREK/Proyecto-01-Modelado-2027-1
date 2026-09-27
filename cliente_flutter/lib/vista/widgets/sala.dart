import 'package:flutter/material.dart';
import '../../controlador/controlador.dart';

void mostrarDialogoNuevaSala(BuildContext context, Controlador controlador) {
  final control = TextEditingController();

  showDialog(
    context: context,
    builder: (dialogContext) {
      return AlertDialog(
        title: const Text('Unirse o Crear Sala'),
        content: TextField(
          controller: control,
          autofocus: true,
          decoration: const InputDecoration(
            hintText: 'Nombre de la sala (Ej. General)',
            border: OutlineInputBorder(),
          ),
          onSubmitted: (nombreSala) {
            _procesarEntrada(dialogContext, controlador, nombreSala);
          },
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialogContext),
            child: const Text('Cancelar'),
          ),
          ElevatedButton(
            onPressed: () {
              _procesarEntrada(dialogContext, controlador, control.text);
            },
            child: const Text('Entrar'),
          ),
        ],
      );
    },
  );
}

void _procesarEntrada(BuildContext context, Controlador controlador, String nombre) {
  final sala = nombre.trim();
  if (sala.isNotEmpty) {
    // Si la sala no existe se creará, si existe simplemente se unirá
    controlador.entrarSala(sala); 
    Navigator.pop(context);
  }
}