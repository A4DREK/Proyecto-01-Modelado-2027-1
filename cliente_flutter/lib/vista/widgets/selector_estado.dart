import 'package:cliente_flutter/modelos/protocolo.dart';
import 'package:flutter/material.dart';

import '../../controlador/controlador.dart';

class SelectorEstado extends StatelessWidget {
  final Controlador controlador;

  //Costructor
  const SelectorEstado({super.key, required this.controlador});

  Color _obetenerColorEstado(String estado) {
    switch (estado.toLowerCase()) {
      case 'active':
        return Colors.green;
      case 'busy':
        return Colors.red;
      case 'away':
        return Colors.orangeAccent;
      default:
        return Colors.grey;
    }
  }

  @override
  Widget build(BuildContext contexto) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16.0),
      child: DropdownButtonHideUnderline(
        child: DropdownButton<EstadoUsuario>(
          value: controlador.miEstado,
          dropdownColor: Colors.deepPurple,
          icon: const Icon(Icons.arrow_drop_down, color: Colors.white),
          onChanged: (EstadoUsuario? nuevoEstado) {
            if (nuevoEstado != null) {
              controlador.cambiarEstado(nuevoEstado);
            }
          },
          items: EstadoUsuario.values.map((EstadoUsuario estado) {
            return DropdownMenuItem<EstadoUsuario>(
              value: estado,
              child: Row(
                children: [
                  Icon(
                    Icons.circle,
                    size: 12,
                    color: _obetenerColorEstado(estado.name),
                  ),
                  const SizedBox(width: 8),
                  Text(
                    estado.name.toUpperCase(),
                    style: const TextStyle(color: Colors.white, fontSize: 14),
                  ),
                ],
              ),
            );
          }).toList(),
        ),
      ),
    );
  }
}
