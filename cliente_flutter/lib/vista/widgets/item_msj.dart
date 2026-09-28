import 'package:flutter/material.dart';
import '../../modelos/mensajes_server.dart';

class ItemMsj extends StatelessWidget {
  final dynamic msj;

  //Constructor fmlekjd
  const ItemMsj({super.key, required this.msj});

  @override
  Widget build(BuildContext contexto) {
    //Si el msj es de texto Publico
    if(msj is PublicTextFromMsj) {
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

    //Si el mensaje es de alguna Sala
    if(msj is RoomTextFromMsj) {
      final esSistema = msj.username == 'Sistema';
      if(esSistema) {
        return Container(
          margin: const EdgeInsets.symmetric(vertical: 6.0),
          alignment: Alignment.center,
          child: Container(
            padding: const EdgeInsets.symmetric(horizontal: 12.0, vertical: 4.0),
            decoration: BoxDecoration(
              color: Colors.grey[200],
              borderRadius: BorderRadius.circular(12),
            ),
            child: Text(
              msj.text,
              style: TextStyle(
                fontSize: 12,
                fontStyle: FontStyle.italic,
                color: Colors.grey[700],
              ),
            ),
          ),
        ); 
      }

      return Padding(
        padding: const EdgeInsets.only(bottom: 4.0),
        child: RichText(
          text: TextSpan(
            style: const TextStyle(color: Colors.black, fontSize: 16),
            children: [
              TextSpan(
                text: '${msj.username}: ',
                style: const TextStyle(fontWeight: FontWeight.bold, color: Colors.deepPurple),
              ),
              TextSpan(text: msj.text),
            ],
          ),
        ),
      );
    }

    //Si el mensaje es Privado
    if (msj is TextFromMsj) {
      return Padding(
        padding: const EdgeInsets.only(bottom: 8.0),
        child: RichText(
          text: TextSpan(
            style: const TextStyle(color: Colors.black, fontSize: 16),
            children: [
              TextSpan(
                text: '[Privado de ${msj.username}]: ',
                style: const TextStyle(fontWeight: FontWeight.bold, color: Colors.purple),
              ),
              TextSpan(text: msj.text),
            ],
          ),
        ),
      );
    }

    return const SizedBox.shrink();
  }
}