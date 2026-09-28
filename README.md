# PROYECTO MODELADO Y PROGRAMACIÓN

## Aplicación de Chat usando programación Asíncrona

Este proyecto de MyP es un chat que usa programación asícrona entre un servidor 
y una serie de clientes los cuales deben de estar conectados todos a la misma red
de internet.

El servidor se realizó en el lenguaje de progrmaación de Rust; mientras que, el cliente
se realizó bajo el FrameWork de Flutter (Dart)

---

## Construcción del proyecto
Se necesita los compiladores de Rust y Flutter para poder correr este proyecto
para revisar que estén instalados utilizar: 

```bash
cargo version
flutter --version
```
Si no se tienen instalados, recominedo checar las páginas oficiales de Rust como de Flutter 
para ver el proceso de instalación

Una vez que se tenga intalados correr el siguiente comando en la carpeta de Server_Rust.
```bash
cargo build
```

### Compilación del Server y Ejecución del Cliente

Para levantar el server solamente tenemos que correr el siguiente comando: 

```bash
RUST_LOG=info cargo run -- --puerto <Puerto>
```
Notar que <Puerto> se refiere al puerto donde se quiera iniciar el server 
Para acabar con el server nada más basta con presionar  **ctrl+c**

Para correr el cliente tenemos dos opciones:
*Nota:* Para ambos casos se requieren de 3 parámetros `<Ip> <Puerto> <Usuario> `

#### Si se quiere ocupar el "Modo desarrollador"
Dentro de la carpeta de cliente_flutter correr el siguiente comando:

```bash
flutter run -d <OS> -a <IP> -a <Puerto> -a <Usuario>
```

#### Si se quiere ocupar un ejecutable
Primero correr: 
```bash
flutter build <OS>
```

Esto permitirá la creación de un archivo ejecutable

Ahora, en la terminal ejecutamos el binario pasando los argumentos de forma directa tq: 

```bash
# La ruta puede variar ligeramente según el nombre de tu proyecto
./build/macos/Build/Products/Release/cliente_flutter <IP> <Puerto> <Usuario>

#Nota, esto puede cambiar para linux, ya que este se encuentra por lo general en
./build/linux/x64/release/bundle/cliente_flutter <IP> <Puerto> <Usuario>