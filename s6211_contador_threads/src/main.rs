use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn main() {
    // criar contador compartilhado com Arc e Mutex 
    let contador = Arc::new(Mutex::new(0));
    //criar um vetor para armazenar os handles das threads 
    let mut handles = vec![];
    // iniciar 10 threads

    for _ in 0..10{
      // criar uma referencia compartilhada para o contador usando Arc
      let contador = Arc::clone(&contador);
      //inicia uma nova thread
      let handle = thread::spawn(move ||{
        //adquire o lock do mutex
        let mut num = contador.lock().unwrap();
        //incrementa o contador
        *num +=1;   
      });  
      //Armazena o handle da thread no vetor
      handles.push(handle);
    }
    
    // aguarda todas as thread terminarem 
    for handle in handles{
        handle.join().unwrap();
        
    }
    // imprime valor final do contador
    println!("o contador final: {}", *contador.lock().unwrap());

    
}
