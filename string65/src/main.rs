fn main(){
    let mut minhaString: String = String::from("Ola meu nom eh Henrique");
    println!("O tamanho dessa string eh {},", minhaString.len());
    println!("A minha esta vazia ? {},", minhaString.is_empty());
    for token in minhaString.split_whitespace(){
        println!("{}", token);
    }
    println!("O nome Henrique esta contido an String? {}", minhaString.contains("Henrique"));
    minhaString.push_str("Bem-vindo a aula ");
    println!("{}", minhaString);

    let mut s = "Hello, ".to_string();
s.push('w');
s.push('o');
s.push('r');
s.push('l');
s.push('d');
s.push('!');

println!("{}", s);


//Você pode iterar sobre uma string usando o método chars():

for c in s.chars() {
    println!("{}", c);
}
//Você também pode iterar sobre uma string usando o método bytes():

for b in s.bytes() {
    println!("{}", b);
}
 /*

 O programa define uma função principal que cria uma variável chamada "minhaString" do tipo "String" com o valor inicial "Ola meu nome eh Henrique". O programa então imprime o comprimento da string e verifica se ela está vazia.

Em seguida, ele usa um loop for para iterar sobre as palavras individuais na string, que são obtidas chamando o método "split_whitespace". O programa então usa o método "contém" para verificar se a string contém a palavra "Henrique", e finalmente acrescenta a string "Bem-vindo a aula " à string original e imprime o resultado.



O código é útil para demonstrar vários métodos úteis para manipulação de strings em Rust e também para verificar se uma determinada palavra está presente em uma string.



Secção Complementar


Uma string em Rust é um tipo de dado que representa uma sequência de caracteres. Ele é denominado como "String" e é diferente do tipo "&str", que é uma referência a uma string. As Strings são imutáveis e alocadas dinamicamente na heap, ao contrário de &str que são alocadas na stack.



Existem várias maneiras de criar uma string em Rust. A maneira mais comum é usando as aspas duplas:

let s = "Hello, world!";
Você também pode criar uma string vazia usando a função String::new():

let s = String::new();
Uma outra maneira de criar uma string é através da conversão de uma slice &str para String com o método to_string():

let s = "Hello, world!".to_string();


Para concatenar strings, você pode usar o operador "+":

let s1 = "Hello, ".to_string();
let s2 = "world!".to_string();
let s3 = s1 + &s2;
Você também pode usar o método push_str():

let mut s1 = "Hello, ".to_string();
let s2 = "world!".to_string();
s1.push_str(&s2);
ou o método push():

let mut s = "Hello, ".to_string();
s.push('w');
s.push('o');
s.push('r');
s.push('l');
s.push('d');
s.push('!');


Você pode iterar sobre uma string usando o método chars():

for c in s.chars() {
    println!("{}", c);
}
Você também pode iterar sobre uma string usando o método bytes():

for b in s.bytes() {
    println!("{}", b);
}
Alguns métodos úteis incluem:

len(): retorna o comprimento da string

is_empty(): retorna true se a string estiver vazia

contains(): verifica se a string contém um determinado substring

replace(): substitui todas as ocorrências de um substring por outro

split(): divide a string em um vetor de substrings baseado em um delimitador






 */
}