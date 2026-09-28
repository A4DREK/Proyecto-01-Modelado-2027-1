import 'package:flutter/material.dart';
import '../../controlador/controlador.dart';

void mostarDialogoInvitacion(BuildContext contexto, Controlador controlador, String roomname) {
  final todosLosUsuarios = controlador.usuariosConectados.keys.toList();
  final usuariosSala = controlador.usuariosPorSala[roomname]?.keys.toList() ?? [];

  final usuariosDisponibles = todosLosUsuarios.where((u) =>
    !usuariosSala.contains(u) && u != controlador.miUsuario
  ).toList();

  if(usuariosDisponibles.isEmpty){
    ScaffoldMessenger.of(contexto).showSnackBar(
      const SnackBar(content: Text('No hay usuarios disponibles para invitar a la sala')),
    );
    return;
  }

  List<String> seleccionados = [];

  showDialog(
    context: contexto,
    builder: (contexto) {
      return StatefulBuilder(
        builder: (contexto, setDialogState) {
          return AlertDialog(
            title: Text('Invitar a: #$roomname'),
            content: SizedBox(
              width: 300,
              height: 300,
              child: ListView.builder(
                itemCount: usuariosDisponibles.length,
                itemBuilder: (contexto, index) {
                  final usuario = usuariosDisponibles[index];
                  return CheckboxListTile(
                    title: Text(usuario),
                    value: seleccionados.contains(usuario),
                    onChanged: (bool? check) {
                      setDialogState(() {
                        if(check == true) {
                          seleccionados.add(usuario);
                        } else {
                          seleccionados.remove(usuario);
                        }
                      });
                    }
                  );
                },
              ),
            ),
            actions: [
              TextButton(
                onPressed: () {
                  if(seleccionados.isNotEmpty){
                    controlador.invitarSala(roomname, seleccionados);
                    Navigator.pop(contexto);
                    ScaffoldMessenger.of(contexto).showSnackBar(
                      SnackBar(content: Text('Invitación enviada a: ${seleccionados.length} usuario(s)')),
                    );
                  }
                },
                child: const Text('Invitar'),
              ),
            ],
          );
        },
      );
    },
  );
}