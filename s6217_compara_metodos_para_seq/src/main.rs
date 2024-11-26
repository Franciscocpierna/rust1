use rayon::prelude::*;
use std::time::{Instant};

fn main() {
   let numeros = vec![1, 2, 3, 4, 5];
   /* soma de elementos dos vetores metodo paralelo
   let soma: u32 = numeros.iter().par_bridge().sum();
   println!("Soma paralela: {}", soma) 
*/
  //medir o tempo da  soma sequencial
  let start_sequencial = Instant::now();
  let soma_sequencial: u32 = numeros.iter().sum(); 
  let duration_sequencial = start_sequencial.elapsed();
  println!("Soma Sequencial: {} (tempo: {:?}) ", soma_sequencial, duration_sequencial);

  //medir o tempo da soma paralela
  let start_paralela = Instant::now();
  let soma_paralela: u32 = numeros.iter().par_bridge().sum(); 
  let duration_paralela = start_sequencial.elapsed();
  println!("Soma Paralela: {} (tempo: {:?}) ", soma_paralela,duration_paralela);
}
