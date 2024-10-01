
fn mediana(numeros: &Vec<i32>)->f64{
   let mut numeros1  = numeros.clone();
   let mut encontra_meio = 0;
   let mut media=0.0;
   let tamanho = numeros.len();

   numeros1.sort();
   println!("{:?}", numeros1);
   encontra_meio = numeros1.len()/2;
   if numeros1.len() % 2 ==0{
      media = (numeros1[encontra_meio]+numeros1[encontra_meio-1]) as f64 /2.0;
      return media;
   }
   numeros1[encontra_meio] as f64


    

}

fn main() {
    let numeros = vec![10,5,8,7,9,0];
    println!("{:.1}",mediana(&numeros));

}

/* professor
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
        let numeros = vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0];
        match mediana(&numeros) {
               Ok(mediana) => println!("Mediana: {:.2}", mediana),
                Err(err) => println!("Erro: {}", err),
         };
}

    */