use rayon::prelude::*;
use std::time::{Instant};

fn is_primo(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..(n / 2 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let numeros: Vec<u32> = (2..50).collect();

 
 
    // Medir o tempo da verificação sequencial de primos
    let start_sequencial = Instant::now();
    let primos_sequencial: Vec<u32> = numeros.iter().cloned().filter(|&x| is_primo(x)).collect();
    let duration_sequencial = start_sequencial.elapsed();

    // Imprimir os primos encontrados sequencialmente e o tempo de execução
    println!("Primos sequencialmente: {:?} (Tempo: {:?})", primos_sequencial, duration_sequencial);

    // Medir o tempo da verificação paralela de primos
    let start_paralelo = Instant::now();
    let primos_paralelo: Vec<u32> = numeros.par_iter().cloned().filter(|&x| is_primo(x)).collect();
    let duration_paralelo = start_paralelo.elapsed();

    // Imprimir os primos encontrados em paralelo e o tempo de execução
    println!("Primos em paralelo: {:?} (Tempo: {:?})", primos_paralelo, duration_paralelo);
}
