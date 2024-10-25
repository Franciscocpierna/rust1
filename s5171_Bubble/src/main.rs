
use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::Read; //esse ou debaixo
//use std::io::prelude::*;

fn swap_array(lista:&mut Vec<i32>,i: usize, j: usize){
    let temp = lista[i];
    lista[i]=lista[j];
    lista[j]=temp;
}

fn main() {
    println!("Esperando para ler arquivo");
    sleep(Duration::from_millis(10000));
    let mut arquivo = File::open("teste.txt").expect("Nao conseguiu ler");
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo).expect("Nao conseguiu ler o arquivo e alocar na variavel conteudo");
    let numeros: Vec<&str> = conteudo.split(",").collect();
    println!("Lidos do Arquivo {:?}", numeros);
    let mut numeros1: Vec<i32> = Vec::new(); 
    for n in 0..numeros.len(){
       numeros1.push(numeros[n].trim().parse::<i32>().unwrap())

    }
   println!("convertidos para vetor i32 {:?}", numeros1); 
   for i in 0..numeros1.len(){
        for j in ((i+1)..numeros1.len()).rev(){
           
            if numeros1[j-1] > numeros1[j]{
               swap_array(&mut numeros1, j-1, j);     

            }
        }
      }     
      println!("ordem crescente {:?}", numeros1);
}



/* professor
fn read_file(filename: &str) -> Vec<i32> {
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let nums: Vec<i32> = contents.split_whitespace()
        .map(|s| s.parse().expect("Unable to parse number"))
        .collect();
    nums
}

fn sort(nums: &mut Vec<i32>) {
    let mut i = 0;
    let mut j = nums.len() - 1;
    while i < j {
        for k in i..j {
            if nums[k] > nums[k+1] {
                let temp = nums[k];
                nums[k] = nums[k+1];
                nums[k+1] = temp;
            }
        }
        j -= 1;
    }
}

fn main() {
    let filename = "nums.txt";
    let mut nums = read_file(filename);
    sort(&mut nums);
    for num in nums {
        println!("{}", num);
    }
}

instruções do programa

Esta tarefa requer que o candidato crie um programa em Rust que use funções, while e for loops para ler uma lista de números inteiros de um arquivo de texto, ordená-los em ordem crescente e imprimi-los na tela.

O programa deve usar a função File::open para abrir o arquivo de texto, o método read_to_string para ler o conteúdo do arquivo para uma string, a função split para converter a string em uma lista de números inteiros e um loop for para percorrer a lista. Um loop while deve ser usado para comparar e trocar os números na lista até que a lista esteja ordenada.



A primeira função, chamada read_file(filename: &str) -> Vec<i32>, é responsável por ler um arquivo de texto especificado pelo nome (filename) e retornar seu conteúdo como um vetor de números inteiros (Vec<i32>). O arquivo é aberto usando o método File::open(), que pode falhar e, portanto, tem o método expect() para lidar com essa falha. O conteúdo do arquivo é então lido para uma string usando o método read_to_string(), que também pode falhar.

O conteúdo da string é dividido em substrings usando o método split_whitespace(), que divide a string em substrings separadas por espaços em branco. Então, cada substring é convertida em um número inteiro usando o método parse() e adicionada a um vetor de números inteiros, que é retornado no final da função.

A segunda função, chamada sort(nums: &mut Vec<i32>), é responsável por ordenar o vetor de números inteiros passado para ela. Ela usa um algoritmo de ordenação chamado "bubble sort", que compara cada número com o próximo e, se necessário, troca seus lugares. O algoritmo continua comparando e trocando até que o vetor esteja completamente ordenado.

A função main é a função principal do programa. Ela define o nome do arquivo a ser lido como "nums.txt" e chama a função read_file para ler os números do arquivo e armazená-los em um vetor. Ela então chama a função sort para ordenar o vetor e, finalmente, usa um loop para imprimir cada número no vetor ordenado.

*/