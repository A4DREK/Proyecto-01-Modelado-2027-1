//Parte de la estrcutura es gracias a la siguiente página
// https://medium.com/@jaimetellezb/flutter-statelesswidget-y-statefulwidget-2996883f2993

import 'dart:io';
import 'package:flutter/material.dart';
import '../controlador/controlador.dart';
import '../modelos/mensajes_server.dart';

class ChatApp extends StatelessWidget {
  final Controlador controlador;

  //Constructor const
  const ChatApp({super.key, required this.controlador});

  @override
  Widget build(BuildContext contexto) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Chat TCP MyP proyecto',
      theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
          useMaterial3: true,
      ),
      home: PantallaPrincipal(controlador: controlador),
    );
  }
}

class PantallaPrincipal extends StatelessWidget {
  final Controlador controlador;
  final TextEditingController _inputControl = TextEditingController();

  //Contructor
  PantallaPrincipal({super.key, required this.controlador});

  @override
  Widget build(BuildContext contexto){
    return ListenableBuilder(
      listenable: controlador,
      builder: (contexto, child) {
        return Scaffold(
          appBar: AppBar(
            title: Text('Chat gnral - ${controlador.miUsuario}'),
            backgroundColor: Colors.deepPurple,
            foregroundColor: Colors.white,

            actions: [
              IconButton(
                icon: const Icon(Icons.exit_to_app),
                onPressed: () {
                  controlador.desconectar();
                  exit(0);
                },
              )
            ],
          ),
          body: Row(
            children: [
              //Parte del panel izquierdo de la lista de usuarios
              Container(
                width: 200,
                color: Colors.indigoAccent[200],
                child: Column(
                  children: [
                    const Padding(
                      padding: EdgeInsets.all(8.0),
                      child: Text('Usuarios', style: TextStyle(fontWeight: FontWeight.bold)),
                    ),
                    const Divider(),
                    Expanded(
                      child: ListView.builder(
                        itemCount: controlador.usuariosConectados.length,
                        itemBuilder: (contexto, index) {
                          String nombre = controlador.usuariosConectados.keys.elementAt(index);
                          String estado = controlador.usuariosConectados[nombre]!.name;

                          return ListTile(
                            leading: Icon(
                              Icons.circle,
                              size: 12,
                              color: estado == 'active' ? Colors.green : Colors.orange,
                            ),
                            title: Text(nombre),
                            subtitle: Text(estado, style: const TextStyle(fontSize: 10)),
                          );
                        },
                      ),
                    )
                  ],
                )
              ),
              Expanded(
                child: Column(
                  children: [
                    Expanded(
                      child: ListView.builder(
                        padding: const EdgeInsets.all(16),
                        itemCount: controlador.historialPublicoChat.length,
                        itemBuilder: (contexto, index) {
                          final msj = controlador.historialPublicoChat[index];

                          if(msj is PublicTextFromMsj) {
                            return Padding(
                              padding: const EdgeInsets.only(bottom: 8.0),
                              child: RichText(
                                text: TextSpan(
                                  style: const TextStyle(color: Colors.black, fontSize: 16),
                                  children: [
                                    TextSpan(
                                      text: '${msj.username}: ',
                                      style: const TextStyle(fontWeight: FontWeight.bold)
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
                              decoration: const InputDecoration(
                                hintText: 'Escribe un msj...',
                                border: OutlineInputBorder(),
                                contentPadding: EdgeInsets.symmetric(horizontal: 16),
                              ),
                              onSubmitted: (texto){
                                _enviarMsj();
                              },
                            ),
                          ),
                          const SizedBox(width: 8,),
                          IconButton(
                            icon: const Icon(Icons.send, color: Colors.pinkAccent),
                            onPressed: _enviarMsj
                          )
                        ]
                      )
                    )
                  ],
                ))
            ],
          )
        );
      },
    );
  }

  void _enviarMsj() {
    final texto = _inputControl.text.trim();
    if(texto.isNotEmpty){
      controlador.mandarTextoPublico(texto);
      _inputControl.clear();
    }
  }
}