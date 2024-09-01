fn lucromaximo(vetor: &[i32])->i32{
  let mut menorvalor= vetor[0];
  let mut maiorvalor = vetor[0];
  let mut pegamenor: Vec<i32>  = Vec::new();
  let mut pegamaior: Vec<i32>  = Vec::new();  
  pegamaior.push(0);
  pegamenor.push(0);
  for n in 1..vetor.len(){
        
        if menorvalor > vetor[n] && n < vetor.len()-1{
            menorvalor = vetor[n];
            pegamenor.push(n as i32);  
        }
             
       
       
        

    }
    maiorvalor = 0;
    for n in pegamenor.len()..vetor.len(){
        if maiorvalor < vetor[n] {
            
            maiorvalor = vetor[n];
            pegamaior.push(n as i32);
        }    

    }
    if  maiorvalor > menorvalor{
         return maiorvalor-menorvalor;
    }else{
        println!("Não tem melhor dia");
        return maiorvalor-menorvalor;
    }

} // fn lucromaximo  

fn main() {
    let  vetor1 = vec![7,1,5,3,6,4];
    let  vetor2 = vec![7,6,4,3,1];
    let mut lucro = lucromaximo(&vetor1); 
    println!("lucro máximo é {}", lucro );
    lucro = lucromaximo(&vetor2); 
    println!("lucro máximo é {}", lucro );
}
/*
fn lucro_maximo(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let mut preco_minimo = prices[0];
    let mut lucro_maximo = 0;

    for preco in prices.iter().skip(1) {
        let lucro_atual = preco - preco_minimo;
        lucro_maximo = lucro_maximo.max(lucro_atual);
        preco_minimo = preco_minimo.min(*preco);
    }

    lucro_maximo
}

fn main() {
    // Exemplo 1
    let precos1 = vec![7, 1, 5, 3, 6, 4];
    let lucro1 = lucro_maximo(precos1.clone());
    println!("Exemplo 1: Lucro Máximo = {}", lucro1);

    // Exemplo 2
    let precos2 = vec![7, 6, 4, 3, 1];
    let lucro2 = lucro_maximo(precos2.clone());
    println!("Exemplo 2: Lucro Máximo = {}", lucro2);
}
*/