
fn media(numeros: &Vec<i32>) -> f64{
    let mut soma = 0;
 
    for i in numeros{
        soma += i;
    }
 
    soma as f64 /  numeros.len() as f64
}

fn mediana(numeros: &Vec<i32>) -> f64{
    let mut numeros_sorted = numeros.clone();
    numeros_sorted.sort();
    println!("O vetor esta em ordem crescente {:?}", numeros_sorted);
 
    let numero_meio = numeros.len() /2;
    if numeros_sorted.len() % 2 == 0{
        return media( &vec![numeros_sorted[numero_meio], numeros_sorted[numero_meio -1]]);
 
    }
 
    numeros_sorted[numero_meio] as f64
}
 
fn main(){
 
    let numeros = vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0];
    println!("{}", media(&numeros));
    println!("{}", mediana(&numeros));
 
}



/*

O código acima define uma função Rust chamada "mediana" que recebe um vetor de números i32 e retorna um valor f64. Dentro da função, ele cria uma nova variável "numeros_sorted" que é uma cópia do vetor de entrada e o ordena em ordem crescente. Em seguida, ele calcula o índice do meio do vetor ordenado dividindo o comprimento do vetor por 2. Se o comprimento do vetor for par, ele pega os dois elementos do meio e calcula a média deles chamando a função "media" passando esses dois elementos em um novo vetor e retornando o resultado. Se o comprimento do vetor for ímpar, ele retorna o valor do elemento do meio. A função principal é definida e ela cria um vetor de números e chama a função "mediana" passando o vetor por referência e imprime o resultado. A função "mediana" calcula o valor mediano dos números fornecidos.






*/