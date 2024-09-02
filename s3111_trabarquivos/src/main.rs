use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write};


fn main() {
    // Abrindo o arquivo e lendo o conteúdo
    let file = File::open("dados.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();
        println!("{}", l);
    }

    // Adicionando novo conteúdo ao arquivo
    let mut file = OpenOptions::new()
        .append(true)
        .open("dados.txt")
        .unwrap();
    let novo_conteudo = "Nova linha de texto\n";
    file.write_all(novo_conteudo.as_bytes()).unwrap();

    // Contando a quantidade de linhas no arquivo
    let file = File::open("dados.txt").unwrap();
    let reader = BufReader::new(file);
    let num_lines = reader.lines().count();
    println!("Número de linhas: {}", num_lines);
}

/*
O código está usando três importações diferentes:



std::fs::{File, OpenOptions}: Essa importação inclui as structs File e OpenOptions do módulo fs (filesystem) da biblioteca padrão do Rust. Essas structs são usadas para abrir e manipular arquivos no sistema de arquivos. A struct File é usada para abrir arquivos para leitura e a struct OpenOptions é usada para abrir arquivos para escrita.

std::io::{BufReader, BufRead, Write}: Essa importação inclui as structs BufReader, BufRead e Write do módulo io (input/output) da biblioteca padrão do Rust. Essas structs são usadas para ler e escrever arquivos de forma mais eficiente. A struct BufReader é usada para ler o arquivo linha por linha, a struct BufRead é usada para ler o conteúdo do arquivo em buffer e a struct Write é usada para escrever dados em um arquivo

std::io é a biblioteca padrão do rust responsavel por lidar com entrada e saída de dados.

Com essas importações, é possível abrir e manipular arquivos no sistema de arquivos, ler e escrever arquivos de forma eficiente e contar o número de linhas em um arquivo.



use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write};

fn main() {
    // Abrindo o arquivo e lendo o conteúdo
    let file = File::open("dados.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();
        println!("{}", l);
    }
O primeiro bloco de código abre o arquivo "dados.txt" usando a struct File e o método open(). O método unwrap() é usado para garantir que o arquivo é aberto corretamente e tratar qualquer erro que possa ocorrer. Em seguida é criado um objeto BufReader passando o arquivo aberto, esse objeto é usado para ler o conteúdo do arquivo linha por linha. A estrutura for é usada para iterar sobre as linhas lidas, cada linha é obtida com o método unwrap() e impressa na tela com o método println!



    // Adicionando novo conteúdo ao arquivo
    let mut file = OpenOptions::new()
        .append(true)
        .open("dados.txt")
        .unwrap();
    let novo_conteudo = "Nova linha de texto\n";
    file.write_all(novo_conteudo.as_bytes()).unwrap();


O segundo bloco de código abre o arquivo "dados.txt" em modo de escrita usando a struct OpenOptions e o método open(). O método append(true) é usado para garantir que o novo conteúdo é adicionado ao final do arquivo. O método unwrap() é usado para garantir que o arquivo é aberto corretamente e tratar qualquer erro que possa ocorrer. Em seguida é definido uma string com o conteúdo a ser adicionado, o método write_all() é usado para escrever essa string no arquivo, o método as_bytes() é usado para transformar a string em um vetor de bytes. O método unwrap() é usado para garantir que a escrita foi realizada corretamente e tratar qualquer erro que possa ocorrer.



    // Contando a quantidade de linhas no arquivo
    let file = File::open("dados.txt").unwrap();
    let reader = BufReader::new(file);
    let num_lines = reader.lines().count();
    println!("Número de linhas: {}", num_lines);
}


O terceiro bloco de código abre novamente o arquivo "dados.txt" usando a struct File e o método open(). O método unwrap() é usado para garantir que o arquivo é aberto corretamente e tratar qualquer erro que possa ocorrer. Em seguida é criado um objeto BufReader passando o arquivo aberto, esse objeto é usado para ler o conteúdo do arquivo linha por linha, então o método count() é chamado no objeto lines para contar a quantidade de linhas no arquivo. O resultado é salvo na variável num_lines e impressa na tela usando o método println!

Esses três blocos de código juntos abrem e manipulam um arquivo de texto, lendo o conteúdo dele, adicionando novo conteúdo e contando a quantidade de linhas no arquivo. Ajudando a entender como funciona a manipulação de arquivos em Rust.
*/