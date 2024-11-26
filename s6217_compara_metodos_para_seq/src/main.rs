use rayon::prelude::*;

fn main() {
   let numeros = vec![1, 2, 3, 4, 5];
   let soma: u32 = numeros.iter().par_bridge().sum();
   println!("Soma paralela: {}", soma) 


}
