import 'dart:io';
import 'package:cliente_flutter/vista/widgets/selector_estado.dart';
import 'package:flutter/material.dart';
import '../controlador/controlador.dart';
import 'widgets/panel_lateral.dart';
import 'widgets/item_msj.dart';
import 'widgets/dialogo_inv.dart';


class ChatApp extends StatelessWidget {
  final Controlador controlador;

  const ChatApp({super.key, required this.controlador});

  @override
  Widget build(BuildContext context) {
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

  const PantallaPrincipal({super.key, required this.controlador});

  @override
  State<PantallaPrincipal> createState() => _PantallaPrincipalState();
}

class _PantallaPrincipalState extends State<PantallaPrincipal> {
  final TextEditingController _inputControl = TextEditingController();
  String? _salaSeleccionada;
  String? _usuarioPrivSeleccionado;

  List<dynamic> _obtenerHistorialActual() {
    if (_usuarioPrivSeleccionado != null) {
      return widget.controlador.chatPrivados[_usuarioPrivSeleccionado] ?? [];
    }
    if (_salaSeleccionada == null) {
      return widget.controlador.historialPublicoChat;
    }
    return widget.controlador.historialSalas[_salaSeleccionada] ?? [];
  }

  void _enviarMsj() {
    final txt = _inputControl.text.trim();
    if (txt.isEmpty) return;

    if (_usuarioPrivSeleccionado != null) {
      widget.controlador.mandarTxtPriv(_usuarioPrivSeleccionado!, txt);
    } else if (_salaSeleccionada == null) {
      widget.controlador.mandarTextoPublico(txt);
    } else {
      widget.controlador.mandarTxtSala(_salaSeleccionada!, txt);
    }

    _inputControl.clear();
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: widget.controlador,
      builder: (context, child) {
        
        final tituloHeader = _usuarioPrivSeleccionado != null
            ? 'Privado con: $_usuarioPrivSeleccionado'
            : _salaSeleccionada == null
                ? 'Chat general - ${widget.controlador.miUsuario}'
                : 'Sala: #$_salaSeleccionada';

        return Scaffold(
          appBar: AppBar(
            title: Text(tituloHeader),
            backgroundColor: Colors.deepPurple,
            foregroundColor: Colors.white,
            actions: [
              if (_salaSeleccionada != null)
                IconButton(
                  icon: const Icon(Icons.person_add_alt_1),
                  tooltip: 'Invitar Usuarios',
                  onPressed: () => mostarDialogoInvitacion(context, widget.controlador, _salaSeleccionada!),
                ),
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
              // PANEL IZQUIERDO 
              PanelLateral(
                controlador: widget.controlador,
                salaSelec: _salaSeleccionada,
                usuarioPrivSelec: _usuarioPrivSeleccionado,
                onSelecSala: (sala) {
                  setState(() {
                    _salaSeleccionada = sala;
                    _usuarioPrivSeleccionado = null;
                  });
                },
                onSelecUsuarioPriv: (usuario) {
                  setState(() {
                    _usuarioPrivSeleccionado = usuario;
                    _salaSeleccionada = null;
                  });
                },
              ),

              // PANEL DERECHO (MENSAJES + ENTRADA)
              Expanded(
                child: Column(
                  children: [
                    Expanded(
                      child: ListView.builder(
                        padding: const EdgeInsets.all(16),
                        itemCount: _obtenerHistorialActual().length,
                        itemBuilder: (context, index) {
                          return ItemMsj(msj: _obtenerHistorialActual()[index]);
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
                                hintText: _usuarioPrivSeleccionado != null
                                    ? 'Mensaje privado para $_usuarioPrivSeleccionado...'
                                    : _salaSeleccionada == null
                                        ? 'Escribe un msj general...'
                                        : 'Escribe un msj en #$_salaSeleccionada...',
                                border: const OutlineInputBorder(),
                                contentPadding: const EdgeInsets.symmetric(horizontal: 16),
                              ),
                              onSubmitted: (_) => _enviarMsj(),
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
}