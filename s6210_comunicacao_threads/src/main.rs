use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn main() {

    //Exemplo1 comunicação entre threads usando canal(channel)
    //criar um canal de comunicação(transmissor e receptor)
    let(tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mensagem = String::from("ola da thread 1");
        tx.send(mensagem).unwrap();
    });
    let recebido = rx.recv().unwrap();

    println!("Mensagem recebida {}", recebido);
    
}
