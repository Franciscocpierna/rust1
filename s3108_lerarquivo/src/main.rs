
use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;
 
 
fn main(){
    println!("Esperando para ler arquivo");
    sleep(Duration::from_millis(10000));
    let mut arquivo = File::open("teste.txt").expect("Nao conseguiu ler");
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo).expect("Nao conseguiu ler o arquivo e alocar na variavel conteudo");
    println!("O conteudo em arquivo eh :\n\n\n\n{}", conteudo);
}

/*
Este código tenta abrir um arquivo chamado "rust_wiki.txt" e ler seu conteúdo para uma string chamada "conteudo". O método "expect" é usado para lidar com qualquer erro que possa ocorrer ao abrir ou ler o arquivo. Se o arquivo for aberto e lido com sucesso, o conteúdo do arquivo é impresso no console usando a macro println!



A primeira linha importa a biblioteca "std::fs::File" do pacote de arquivos padrão do Rust. Isso permite que o código abra arquivos no sistema de arquivos.

A segunda linha importa a biblioteca "std::io::prelude::*" do pacote de entrada e saída padrão do Rust. Isso permite que o código leia o conteúdo de um arquivo.

A função "main" é declarada e dentro dela, uma variável mutável chamada "arquivo" é iniciada usando "File::open", passando como parametro "rust_wiki.txt", que é o nome do arquivo que será aberto, e caso não consiga abrir o arquivo, será retornado uma mensagem de erro "Nao conseguiu ler" usando "expect".

A seguir, uma outra variável mutável chamada "conteudo" é iniciada como uma string vazia.

A função "read_to_string" é chamada no arquivo aberto, passando como parametro a variavel "conteudo" para armazenar o conteudo lido, caso não consiga ler o arquivo, será retornado uma mensagem de erro "Nao conseguiu ler o arquivo e alocar na variavel conteudo" usando "expect".

Por fim, usando a macro println! é impresso na tela o conteudo armazenado na variavel "conteudo", com uma mensagem "O conteudo em arquivo eh :\n\n{}".

Em resumo, o código abre um arquivo chamado "rust_wiki.txt", lê o conteúdo do arquivo e armazena em uma variável "conteudo" e finalmente imprime o conteúdo do arquivo na tela.





*/