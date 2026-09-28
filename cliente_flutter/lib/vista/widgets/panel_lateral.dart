import 'package:flutter/material.dart';

import '../../controlador/controlador.dart';
import 'sala.dart';

class PanelLateral extends StatelessWidget {
  final Controlador controlador;
  final String? salaSelec;
  final String? usuarioPrivSelec;
  final Function(String?) onSelecSala;
  final Function(String?) onSelecUsuarioPriv;

  // El constructor del Panel Lateral
  const PanelLateral({
    super.key,
    required this.controlador,
    required this.salaSelec,
    required this.usuarioPrivSelec,
    required this.onSelecSala,
    required this.onSelecUsuarioPriv,
  });

  // Se devuelve la lista de los usuarios activos
  List<String> _obtenerUsuariosActivos() {
    if (salaSelec == null) {
      return controlador.usuariosConectados.keys
          .where((u) => u != controlador.miUsuario)
          .toList();
    }
    return controlador.usuariosPorSala[salaSelec]?.keys.toList() ?? [];
  }

  @override
  Widget build(BuildContext contexto) {
    return ListenableBuilder(
      listenable: controlador,
      builder: (contexto, _) {
        return Container(
          width: 200,
          color: Colors.indigoAccent[200],
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Encabezado de las salas
              Padding(
                padding: const EdgeInsets.symmetric(
                  horizontal: 8.0,
                  vertical: 4.0,
                ),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text(
                      'Salas',
                      style: TextStyle(fontWeight: FontWeight.bold),
                    ),
                    IconButton(
                      icon: const Icon(Icons.add, size: 20),
                      tooltip: 'Unirse o crear Sala',
                      onPressed: () async {
                        final nuevaSala = await mostrarDialogosSala(
                          contexto,
                          controlador,
                        );
                        if (nuevaSala != null) {
                          if (nuevaSala.accion == AccionSala.crear) {
                            controlador.crearSala(nuevaSala.nombre);
                          } else {
                            controlador.entrarSala(nuevaSala.nombre);
                          }
                          onSelecSala(nuevaSala.nombre);
                        }
                      },
                    ),
                  ],
                ),
              ),

              // Parte del chat general
              ListTile(
                dense: true,
                selected: salaSelec == null && usuarioPrivSelec == null,
                selectedTileColor: Colors.deepPurple.withValues(alpha: 0.2),
                leading: const Icon(Icons.public, size: 18),
                title: const Text('Chat Generalisimo'),
                onTap: () {
                  onSelecUsuarioPriv(null);
                  onSelecSala(null);
                },
              ),

              // Para la parte de la sección de las invitaciones
              if (controlador.invitacionesPendientes.isNotEmpty) ...[
                const Divider(),
                const Padding(
                  padding: EdgeInsets.symmetric(horizontal: 8.0, vertical: 4.0),
                  child: Text(
                    'Invitaciones',
                    style: TextStyle(
                      fontWeight: FontWeight.bold,
                      color: Colors.orange,
                    ),
                  ),
                ),
                ...controlador.invitacionesPendientes.map((inv) {
                  return ListTile(
                    dense: true,
                    leading: const Icon(
                      Icons.mail,
                      color: Colors.orange,
                      size: 18,
                    ),
                    title: Text(
                      inv.roomname,
                      style: const TextStyle(fontSize: 12),
                    ),
                    trailing: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        IconButton(
                          icon: const Icon(
                            Icons.check_circle,
                            color: Colors.green,
                            size: 20,
                          ),
                          tooltip: 'Unirse',
                          onPressed: () {
                            controlador.entrarSala(inv.roomname);
                            controlador.invitacionesPendientes.removeWhere(
                              (i) => i.roomname == inv.roomname,
                            );
                            onSelecSala(inv.roomname);
                          },
                        ),
                        IconButton(
                          icon: const Icon(
                            Icons.cancel,
                            color: Colors.redAccent,
                            size: 20,
                          ),
                          tooltip: 'Rechazar',
                          onPressed: () {
                            controlador.invitacionesPendientes.removeWhere(
                              (i) => i.roomname == inv.roomname,
                            );
                            onSelecSala(salaSelec);
                          },
                        ),
                      ],
                    ),
                  );
                }),
                const Divider(),
              ],

              // Lista de las salas Activas
              ...controlador.usuariosPorSala.keys.map((nombreSala) {
                final esSeleccionada = salaSelec == nombreSala;
                return ListTile(
                  dense: true,
                  selected: esSeleccionada,
                  selectedTileColor: Colors.deepPurple.withValues(alpha: 0.3),
                  leading: const Icon(Icons.tag, size: 18),
                  title: Text(nombreSala),
                  trailing: IconButton(
                    icon: const Icon(Icons.close, size: 14),
                    tooltip: 'Salir de la sala',
                    onPressed: () => controlador.salirSala(nombreSala),
                  ),
                  onTap: () {
                    onSelecUsuarioPriv(null);
                    onSelecSala(nombreSala);
                  },
                );
              }),
              const Divider(),

              // Lugar donde van a estar los usuarios
              Padding(
                padding: const EdgeInsets.all(8.0),
                child: Text(
                  salaSelec == null ? 'Usuarios' : 'En #$salaSelec',
                  style: const TextStyle(fontWeight: FontWeight.bold),
                ),
              ),
              Expanded(
                child: ListView(
                  children: _obtenerUsuariosActivos().map((nombre) {
                    final estado =
                        controlador.usuariosConectados[nombre]?.name ??
                        'active';
                    final esSelec = usuarioPrivSelec == nombre;

                    return ListTile(
                      dense: true,
                      selected: esSelec,
                      selectedTileColor: Colors.deepPurpleAccent.withValues(
                        alpha: 0.2,
                      ),
                      leading: Icon(
                        Icons.circle,
                        size: 10,
                        color: estado == 'active'
                            ? Colors.green
                            : Colors.orange,
                      ),
                      title: Text(nombre),
                      subtitle: Text(
                        estado,
                        style: const TextStyle(fontSize: 10),
                      ),
                      onTap: () {
                        onSelecSala(null);
                        onSelecUsuarioPriv(nombre);
                      },
                    );
                  }).toList(),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
