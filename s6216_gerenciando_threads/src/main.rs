use std::thread;
use std::sync::mpsc;
use rand::Rng;

fn main() {
    // Criando um canal de comunicação (transmissor e receptor)
    let (transmissor, receptor) = mpsc::channel();

    // Criando um vetor para armazenar as handles das threads
    let mut handles = vec![];

    // Inicia 5 threads
    for id in 0..5 {
        // Cria uma referência compartilhada para o transmissor usando Arc
        let transmissor = mpsc::Sender::clone(&transmissor);

        // Inicia uma nova thread
        let handle = thread::spawn(move || {
            // Gerando um número aleatório
            let numero = rand::thread_rng().gen_range(1..=10);

            // Calcula o quadrado do número
            let resultado = numero * numero;

            // Envia o resultado para o canal
            transmissor.send((id, resultado)).unwrap();
        });

        // Armazena a handle da thread no vetor
        handles.push(handle);
    }

    // Aguarda todas as threads terminarem e coleta os resultados
    let mut resultados = vec![];
    for _ in 0..5 {
        let (id, resultado) = receptor.recv().unwrap();
        resultados.push((id, resultado));
    }

    // Imprime os resultados
    for (id, resultado) in resultados {
        println!("Thread {} - Quadrado: {}", id, resultado);
    }

    // Aguarda todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }
}