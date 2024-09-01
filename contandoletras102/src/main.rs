
fn maiusculas(mais: &str) -> i32 {
    let mut cont=0;
    for  (n,f) in mais.chars().enumerate(){
    
        if mais.chars().nth(n).unwrap().is_uppercase(){
            cont+=1;
        }
    }    
    cont  
    

}   

fn main() {
    let mais = "Me Encontre Sou Maiscula";
    let contamais = maiusculas(mais);
    println!("Número de letras maiúsculas: {}", contamais);

}



/* professor


fn conta_maiusculas(texto: &str) -> u32 {
    let mut contador = 0;
    for caractere in texto.chars() {
        if caractere.is_uppercase() {
            contador += 1;
        }
    }
    contador
}

fn main() {
    let texto = "Este é um Texto de Exemplo";
    let contador = conta_maiusculas(texto);
    println!("Número de letras maiúsculas: {}", contador);
}

Escreva uma função em Rust que receba uma string e retorne o número de letras maiúsculas presentes na string.

Dicas:

Lembre-se de especificar o tipo de dado da string e da variável que armazena o número de letras maiúsculas.

Você pode usar o método "is_uppercase()" para verificar se uma letra é maiúscula.

Lembre-se de que as strings em Rust são codificadas como uma sequência de bytes, então, para acessar cada caractere deve-se usar os métodos .chars() ou .bytes()




A função conta_maiusculas recebe uma string como parâmetro e retorna um número inteiro sem sinal de 32 bits. Dentro da função, criamos uma variável contador iniciada com zero, e usamos um loop for percorrendo os caracteres da string usando o método .chars(). Dentro do loop, verificamos se o caractere é maiúsculo usando o método is_uppercase() e se for verdadeiro incrementamos o contador. Por fim, a função retorna o valor de contador que é o número de letras maiúsculas na string. Na função main, criamos uma string de exemplo chamada texto e chamamos a função conta_maiusculas passando essa string como argumento. Armazenamos o resultado da função em uma variável contador e imprimimos o resultado na tela.
*/