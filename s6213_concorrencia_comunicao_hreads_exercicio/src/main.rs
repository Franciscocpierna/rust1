use std::thread;
use std::sync::mpsc;
use rand::Rng;
 
fn main() {
    let mut x=0;
    let(tx,rx) = mpsc::channel();
   
    thread::spawn(move || {
        let mut valores_randomicos = rand::thread_rng();
        while x != 10{ 
            let mut valores = valores_randomicos.gen_range(1..5);
            tx.send(valores).unwrap();
            println!("Produtor gerou: {}", valores);
            thread::sleep_ms(1000);   
            x+=1;
    }
   });
    x=0;       
    while x != 10{ 
        let recebido = rx.recv().unwrap();
        let resultado = recebido * recebido;
        println!("Consumidor recebeu e calculou: {}", resultado);  
        x+=1;
    } 

}

/*use std::thread;
use std::sync::mpsc;
use rand::Rng;

fn main() {
    // Criando um canal de comunicação (transmissor e receptor)
    let (transmissor, receptor) = mpsc::channel();

    // Criando uma thread para o produtor
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            // Gerando números aleatórios e enviando para o canal
            let numero = rng.gen_range(1..101);
            transmissor.send(numero).unwrap();
            println!("Produtor gerou: {}", numero);
            // Simulando algum trabalho
            thread::sleep_ms(500);
        }
    });

    // Thread principal atuando como consumidor
    for _ in 0..5 {
        // Recebendo números do canal
        let recebido = receptor.recv().unwrap();
        // Calculando o quadrado e exibindo o resultado
        let resultado = recebido * recebido;
        println!("Consumidor recebeu e calculou: {}", resultado);
    }
}
*/
