use std::io;
 
fn main(){
 
    let mut mensagem_usuario = String::new();
    println!("Ei usuario. Digite algo:\n\n ");
 
 
 
    match io::stdin().read_line(&mut mensagem_usuario){
        Ok(_) => println!("Sucesso. Vc digitou {}", mensagem_usuario.to_uppercase()),
        Err(e) => println!("Deu erro. O erro eh {}", e)
    }
}

/*
Este código em Rust define uma função chamada "main", que é o ponto de entrada do programa. Dentro da função, ele declara uma variável "mensagem_usuario" do tipo "String" e a inicializa como uma string vazia. O programa então imprime uma mensagem pedindo ao usuário para digitar algum texto.

O código usa a entrada padrão (stdin) do módulo io para ler uma linha de texto do usuário e armazená-la na variável "mensagem_usuario". O método read_line retorna um tipo Result, que é uma enumeração que pode ser "Ok" ou "Err".

O código, então, usa uma declaração "match" para verificar o resultado do método read_line. Se o resultado for "Ok", isso significa que a entrada foi lida com sucesso e o programa imprime uma mensagem dizendo "Sucesso. você digitou (entrada do usuário)". Se o resultado for "Err", isso significa que ocorreu um erro ao ler a entrada e o programa imprime uma mensagem dizendo "Erro. O erro é (mensagem de erro)".

Este código permite que o programa leia uma linha de texto do usuário e gerencie a possibilidade de um erro ocorrer ao ler a entrada. Ele usa a declaração "match" para verificar o tipo Result e tomar a ação apropriada com base no resultado.

*/

