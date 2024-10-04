

use std::collections::HashMap;

fn moda(numeros: &Vec<i32>){
    let mut hmap = HashMap::new();
    let mut h1map = HashMap::new();
    let mut contamoda=0;
    for i in numeros{
        let contar = hmap.entry(i).or_insert(0);
        *contar += 1;
        
    }
    println!("{:?}",&hmap);
    let mut maior_valor=0;
    let mut maior_key = 0;
    for (key, value) in hmap{
          if value > 1{
            h1map.insert(key,value);
                       
          } 
          if value > maior_valor{
            maior_valor=value;
            maior_key= *key;
          }
    }
    println!("{:?}",h1map);
    for (key, value) in h1map{
        if value == maior_valor{
            contamoda=contamoda+1;
            if contamoda == 1{
               println!("A moda eh: {} e valor {} ",maior_key,maior_valor);
            }
            if contamoda > 1{         
                println!("foram  encontradas {} as modas com valor {} ",contamoda, maior_valor)
                
            }
        }
        
    }
    
        
    
    
   
}







fn main() {
    let numeros = vec![1,1,3,3,3,0,0,4,9,9,9];
    moda(&numeros) 
}

/*
use std::collections::HashMap;

fn moda(numeros: &[i32]) -> Option<i32> {
    let mut map = HashMap::new();

    // Contar as ocorrências de cada número
    for &i in numeros {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // Encontrar a chave com o maior valor (moda)
    let mut chave_moda = None;
    let mut maior_contagem = 0;

    for (chave, &valor) in &map {
        if valor > maior_contagem {
            maior_contagem = valor;
            chave_moda = Some(*chave);
        }
    }

    // Verificar se há empates (se múltiplos números têm a mesma contagem mais alta)
    let mut chaves_empate = vec![];
    for (chave, &valor) in &map {
        if valor == maior_contagem {
            chaves_empate.push(*chave);
        }
    }

    // Lidar com o caso em que há um empate (retornar None em caso de empate)
    if chaves_empate.len() > 1 {
        None
    } else {
        chave_moda
    }
}

fn main() {
    let numeros = vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0];
    match moda(&numeros) {
        Some(moda) => println!("Moda: {}", moda),
        None => println!("Existem múltiplas modas."),
    };
}

*/