use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn main() {
   let handles: Vec<_> = (0..5).map(|i|{
      thread::spawn(move ||{
        thread::sleep(Duration::from_secs(i as u64));
        println!("thread {} finalizada",i);
        i*i
      })
   }).collect(); 
   let resultados: Vec<_> = handles.into_iter().map(|handle| handle.join().unwrap()).collect();
    println!("Resultados: {:?}", resultados);
}
