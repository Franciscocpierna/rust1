use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;
 

fn main(){
    let mut arquivo = File::create("teste.txt")
        .expect("Nao conseguiu criar o arquivo");
 
    arquivo.write_all(b"Arquivo de teste sendo criado").
        expect("Nao conseguiu ler o arquivo e alocar na variavel conteudo");
        println!("Esperando para ler arquivo");
        sleep(Duration::from_millis(10000));
        let mut arquivo = File::open("teste.txt").expect("Nao conseguiu ler");
        let mut conteudo = String::new();
        arquivo.read_to_string(&mut conteudo).expect("Nao conseguiu ler o arquivo e alocar na variavel conteudo");
        println!("O conteudo em arquivo eh :\n\n\n\n{}", conteudo);
        
    
}