// Estructura adaptada con StatefulWidget
// https://medium.com/@jaimetellezb/flutter-statelesswidget-y-statefulwidget-2996883f2993

import 'dart:io';
import 'package:flutter/material.dart';
import '../controlador/controlador.dart';
import '../modelos/mensajes_server.dart';
import 'widgets/selector_estado.dart';
import 'widgets/sala.dart';

class ChatApp extends StatelessWidget {
  final Controlador controlador;

  //Construcor del Chat App
  const ChatApp({super.key, required this.controlador});

  @override
  Widget build(BuildContext contexto) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Chat TCP MyP',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: PantallaPrincipal(controlador: controlador),
    );
  }
}

class PantallaPrincipal extends StatefulWidget {
  final Controlador controlador;

  //Constructor de la Pantalla principal
  const PantallaPrincipal({super.key, required this.controlador});

  @override
  State<PantallaPrincipal> createState() => _PantallaPrincipalState();

}

class _PantallaPrincipalState extends State<PantallaPrincipal> {
  final TextEditingController _inputControl = TextEditingController();
  
  // null = Chat General || "NombreSala" = Sala seleccionada
  String? _salaSeleccionada;

  @override
  Widget build(BuildContext contexto) {
    return ListenableBuilder(
      listenable: widget.controlador,
      builder: (contexto, child) {
        // Si la sala activa ya no existe en el controlador regresamos al chat general
        if (_salaSeleccionada != null && 
            !widget.controlador.usuariosPorSala.containsKey(_salaSeleccionada)) {
          _salaSeleccionada = null;
        }

        return Scaffold(
          appBar: AppBar(
            title: Text(_salaSeleccionada == null
                ? 'Chat gnral - ${widget.controlador.miUsuario}'
                : 'Sala: #$_salaSeleccionada'),
            backgroundColor: Colors.deepPurple,
            foregroundColor: Colors.white,
            actions: [
              SelectorEstado(controlador: widget.controlador),
              IconButton(
                icon: const Icon(Icons.exit_to_app),
                tooltip: 'Desconectar',
                onPressed: () {
                  widget.controlador.desconectar();
                  exit(0);
                },
              )
            ],
          ),
          body: Row(
            children: [
              // PANEL IZQUIERDO: Salas y Lista de Usuarios
              Container(
                width: 200,
                color: Colors.indigoAccent[200],
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    // Parte de las salas
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 8.0, vertical: 4.0),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          const Text('Salas', style: TextStyle(fontWeight: FontWeight.bold)),
                          IconButton(
                            icon: const Icon(Icons.add, size: 20),
                            tooltip: 'Unirse o Crear Sala',
                            onPressed: () async {
                              // Llamada limpia al widget externo de diálogo
                              final nuevaSala = await mostrarDialogosSala(contexto, widget.controlador);
                              if (nuevaSala != null) {
                                setState(() {
                                  _salaSeleccionada = nuevaSala;
                                });
                              }
                            },
                          ),
                        ],
                      ),
                    ),

                    // Opción fija: Chat General
                    ListTile(
                      dense: true,
                      selected: _salaSeleccionada == null,
                      selectedTileColor: Colors.deepPurple.withValues(),
                      leading: const Icon(Icons.public, size: 18),
                      title: const Text('Chat General'),
                      onTap: () {
                        setState(() {
                          _salaSeleccionada = null;
                        });
                      },
                    ),

                    // Lista de Salas Activas
                    ...widget.controlador.usuariosPorSala.keys.map((nombreSala) {
                      final esSeleccionada = _salaSeleccionada == nombreSala;
                      return ListTile(
                        dense: true,
                        selected: esSeleccionada,
                        selectedTileColor: Colors.deepPurple.withValues(),
                        leading: const Icon(Icons.tag, size: 18),
                        title: Text(nombreSala),
                        trailing: IconButton(
                          icon: const Icon(Icons.close, size: 14),
                          tooltip: 'Salir de la sala',
                          onPressed: () {
                            widget.controlador.salirSala(nombreSala); // O leaveRoom(nombreSala)
                          },
                        ),
                        onTap: () {
                          setState(() {
                            _salaSeleccionada = nombreSala;
                          });
                        },
                      );
                    }),

                    const Divider(),

                    // --- SECCIÓN DE USUARIOS ---
                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Text(
                        _salaSeleccionada == null ? 'Usuarios' : 'En #$_salaSeleccionada',
                        style: const TextStyle(fontWeight: FontWeight.bold),
                      ),
                    ),

                    Expanded(
                      child: ListView(
                        children: _obtenerUsuariosActivos().map((nombre) {
                          final estado = widget.controlador.usuariosConectados[nombre]?.name ?? 'active';
                          return ListTile(
                            dense: true,
                            leading: Icon(
                              Icons.circle,
                              size: 10,
                              color: estado == 'active' ? Colors.green : Colors.orange,
                            ),
                            title: Text(nombre),
                            subtitle: Text(estado, style: const TextStyle(fontSize: 10)),
                          );
                        }).toList(),
                      ),
                    )
                  ],
                ),
              ),

              // PANEL DERECHO: Historial y Entrada de Texto
              Expanded(
                child: Column(
                  children: [
                    Expanded(
                      child: ListView.builder(
                        padding: const EdgeInsets.all(16),
                        itemCount: _obtenerHistorialActual().length,
                        itemBuilder: (contexto, index) {
                          final msj = _obtenerHistorialActual()[index];

                          if (msj is PublicTextFromMsj) {
                            return Padding(
                              padding: const EdgeInsets.only(bottom: 8.0),
                              child: RichText(
                                text: TextSpan(
                                  style: const TextStyle(color: Colors.black, fontSize: 16),
                                  children: [
                                    TextSpan(
                                      text: '${msj.username}: ',
                                      style: const TextStyle(fontWeight: FontWeight.bold),
                                    ),
                                    TextSpan(text: msj.text),
                                  ],
                                ),
                              ),
                            );
                          }
                          return const SizedBox.shrink();
                        },
                      ),
                    ),
                    const Divider(height: 1),

                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Row(
                        children: [
                          Expanded(
                            child: TextField(
                              controller: _inputControl,
                              decoration: InputDecoration(
                                hintText: _salaSeleccionada == null
                                    ? 'Escribe un msj general...'
                                    : 'Escribe un msj en #$_salaSeleccionada...',
                                border: const OutlineInputBorder(),
                                contentPadding: const EdgeInsets.symmetric(horizontal: 16),
                              ),
                              onSubmitted: (texto) => _enviarMsj(),
                            ),
                          ),
                          const SizedBox(width: 8),
                          IconButton(
                            icon: const Icon(Icons.send, color: Colors.pinkAccent),
                            onPressed: _enviarMsj,
                          )
                        ],
                      ),
                    )
                  ],
                ),
              )
            ],
          ),
        );
      },
    );
  }

  List<String> _obtenerUsuariosActivos() {
    if(_salaSeleccionada == null) {
      return widget.controlador.usuariosConectados.keys.toList();
    }

    return widget.controlador.usuariosPorSala[_salaSeleccionada]?.keys.toList() ?? <String>[];
  }

  List<dynamic> _obtenerHistorialActual() {
    if(_salaSeleccionada == null) {
      return widget.controlador.historialPublicoChat;
    }
    return widget.controlador.historialSalas[_salaSeleccionada] ?? [];
  }

  void _enviarMsj() {
    final txt = _inputControl.text.trim();
    if(txt.isNotEmpty && _salaSeleccionada == null) {
      widget.controlador.mandarTextoPublico(txt);
    }else {
      widget.controlador.mandarTxtSala(_salaSeleccionada!, txt);
    }
    _inputControl.clear();
  }
}