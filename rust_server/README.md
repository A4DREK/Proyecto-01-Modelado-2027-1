========================================
RUST_SERVER (SERVIDOR TCP PROYECTO MYP)
========================================

Para poder correr este servidor es necesario: 
1.- Tener instalado Rust y Cargo
    Para comrpbar que se tenga el compilador del lenguaje 
    escribir en terminal 
    rustc --version && cargo --version

2.- Una vez clonado el repositorio de GitHub, se tiene que correr el comando
    cargo build 

3.- Para correr el programa y alzar el serviodr solamente es necesario utilizar el comando
    cargo run 
    Sin embargo, para poder ver todos los logs que recivirá el server, se recomienda el uso de:
    RUST_LOG=info cargo run -- --puerto (Puerto que se quiera utilizar)
    El puerto por defecto del server es el 1234

4.- Para poder conectar sin uso del cliente, se puede ocupar nc o telnet, con la dirección IP de
    la conexión en donde estén las computadoras
    Oucpar el comando 
    telnet (Dirección IP) (Puerto)

5.- Si se quiere realizar la prueba de estrés
    Utilzar los siguientes comandos
    ulimit -n 4096
    RUST_LOG=info cargo test --test test_estres -- --nocapture
