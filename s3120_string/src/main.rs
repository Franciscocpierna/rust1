fn main(){
 
    {
        let minha_string = String::from("Oi meu nome eh Joao");
        println!("{}", minha_string);
        println!("{}", minha_string.replace("Joao", "Henrique"));
 
    }
 
    {
        let minha_string = String::from("Fui hoje \nao mercado \ncomprar arroz");
 
        for i in minha_string.lines(){
            println!("{}", i);
        }
    }
 
    {
        let minha_string = String::from("minha+sogra+eh+muito+feliz");
        let token: Vec<&str> = minha_string.split("+").collect();
        println!("{:?}", token);
        println!("{}", token[1]);
    } 
 
    {
        let minha_string = String::from("    Meu nome eh Joao    ");
        println!("{}", minha_string);
        println!("{}", minha_string.trim());
 
    }
 
    {
        let minha_string = String::from("Deixe uma avaliação de 5 estrelas");
        match minha_string.chars().nth(6){
 
            Some(c) => println!("Sucesso! o carcter da sexta posição eh {}", c),
            None => println!("Erro! Nao existe o carcter da sexta posição")
        }
 
    }
}
/*
O código acima demonstra algumas operações comuns com strings em Rust.



O primeiro bloco de código cria uma string "minha_string" e usa o método "replace" para substituir uma parte da string ("Joao" por "Henrique"). Ele imprime a string original e a string modificada.

O segundo bloco cria outra string "minha_string" com várias linhas e usa o método "lines" para iterar sobre cada linha da string e imprimi-las uma por vez

O terceiro bloco cria outra string "minha_string" e usa o método "split" para separar a string em uma coleção de substrings usando o caractere "+" como separador. Ele usa o método "collect" para transformar a coleção em um vetor de strings "token" e imprime o vetor e o segundo elemento do vetor.

O quarto bloco cria outra string "minha_string" com espaços em branco no início e no final, usa o método "trim" para remover esses espaços e imprime a string original e a string formatada

O quinto bloco cria outra string "minha_string" e usa o método "chars" para obter uma coleção de caracteres de string, e o método "nth" para obter o sexto caractere. Ele usa um "match" para testar se o caractere existe ou não e imprime o caractere ou uma mensagem de erro, respectivamente.

Todas as operações são realizadas usando os métodos associados às strings, sem a necessidade de slices.



Leitura Complementar


Definição: Strings em Rust são uma das estruturas de dados mais utilizadas na linguagem. Elas são representadas pela struct String e oferecem uma série de métodos úteis para a manipulação de strings.

Criação de strings: Para criar uma string em Rust, você pode usar o construtor String::new() para criar uma string vazia ou String::from("string") para criar uma string a partir de uma string literal. Além disso, você também pode usar a sintaxe de string literal "string" diretamente.

Concatenação de strings: Para concatenar strings, você pode usar o método push_str() ou o operador +. Por exemplo:

let mut s1 = String::from("Oi, ");
let s2 = String::from("mundo!");
s1.push_str(&s2);
println!("{}", s1); // "Oi, mundo!"
Indexação: Para acessar caracteres individuais de uma string, você pode usar o operador []. No entanto, essa operação é insegura e pode gerar um erro de indexação se o índice for inválido. Uma alternativa segura é usar o método chars() para iterar sobre os caracteres de uma string.

Busca e substituição: Para buscar e substituir partes de uma string, você pode usar o método find() para encontrar a primeira ocorrência de uma substring e o método replace() para substituí-la.

Fatiamento: Para criar slices de strings, você pode usar o operador & seguido dos índices inicial e final da sub-sequência desejada. Por exemplo, se você tem uma string "hello" e deseja criar um slice com as letras "ell", você pode escrever &s[1..3].

Outros métodos úteis: Além desses métodos, as strings em Rust também oferecem métodos para remover espaços em branco no início e no final de uma string (trim()), dividir uma string em uma coleção de substrings (split()), iterar sobre as linhas de uma string (lines()), entre outros.

Conclusão: As strings em Rust são uma estrutura de dados poderosa e flexível, e os métodos disponíveis tornam fácil realizar a maioria das tarefas comuns



*/