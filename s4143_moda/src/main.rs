
use std::collections::HashMap;



fn moda(numeros: &Vec<i32>) -> i32{
    let mut map = HashMap::new();
    for i in numeros{
        let contar = map.entry(i).or_insert(0);
        *contar += 1;
        
    }
    println!("{:?}",map);
    let mut maior_valor=0;
    let mut maior_key = 0;
    for (key, value) in map{
          if value > maior_valor{
            maior_valor=value;
            maior_key= *key;
          }
    }
    maior_key
}





fn mediana(numeros: &Vec<i32>) -> Result<f64, &'static str> {
    if numeros.is_empty() {
        return Err("O vetor está vazio.");
    }

    let mut numeros_sorted = numeros.clone();
    numeros_sorted.sort();

    println!("O vetor está em ordem crescente: {:?}", numeros_sorted);

    let numero_meio = numeros_sorted.len() / 2;
    match numeros_sorted.len() % 2 {
        0 => {
            // Caso de número par de elementos
            let media = media(&vec![
                numeros_sorted[numero_meio] as f64,
                numeros_sorted[numero_meio - 1] as f64,
            ]);
            Ok(media)
        }
        _ => {
            // Caso de número ímpar de elementos
            Ok(numeros_sorted[numero_meio] as f64)
        }
    }
}

fn media(numeros: &Vec<f64>) -> f64 {
    let soma: f64 = numeros.iter().sum();
    let quantidade = numeros.len() as f64;
    soma / quantidade
}


fn main() {
       // let numeros = vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0];
       // let numeros =vec![0,1,2,2,3,4];
       let numeros =vec![0,1,0,0,3,4];

       match mediana(&numeros) {
               Ok(mediana) => println!("Mediana: {:.2}", mediana),
                Err(err) => println!("Erro: {}", err),
         };
         //moda(&numeros);
         println!("A Moda eh : {}", moda(&numeros));
}