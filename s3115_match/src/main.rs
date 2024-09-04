fn main(){
 
    let nome = "Rodrigo";
 
    match nome{
 
        "Joao" => println!("Joao eh dentista"),
        "Rodrigo" => println!("Rodrigo eh programador"),
        _ => println!("Eu nao sei sua profissao"),
    } 
}
/*
Este código em Rust define uma função chamada "main", que é o ponto de entrada do programa. Dentro da função, ele define uma variável "nome" com o valor de string "Rodrigo". O código usa uma declaração "match", que é similar a uma declaração "switch" em outras linguagens de programação, para comparar o valor de "nome" com diferentes valores de string e executar um bloco específico de código dependendo da correspondência.

Neste caso, a declaração "match" compara o valor de "nome" com a string "João". Se corresponder, o programa imprime "Joao eh dentista" (João é dentista). Se o valor de "nome" corresponder a "Rodrigo", ele imprime "Rodrigo eh programador" (Rodrigo é programador). Se não corresponder a nenhum dos casos anteriores, ele executará o código no caso "_", que imprime "Eu não sei sua profissão".

Este código específico imprimirá "Rodrigo é programador", pois o valor de "nome" é "Rodrigo" e corresponde ao segundo caso na declaração "match".


Leitura Complementar


O "pattern match" em Rust é uma estrutura de controle que permite comparar um valor com diferentes padrões e executar um bloco de código específico de acordo com a correspondência. Ele é similar ao "switch" em outras linguagens de programação, mas é mais flexível e poderoso.

Para usar o "pattern match", você usa a sintaxe "match seguido pelo valor a ser comparado e entre chaves, você define os diferentes padrões e os blocos de código correspondentes. Por exemplo:



   valor1 => println!("valor1"),
    valor2 => println!("valor2"),
    _ => println!("outro valor"),
}```


Cada padrão é seguido por "=>" e então o código que deve ser executado se o valor corresponder a esse padrão. O último padrão é "_", que é um "catch-all" que é usado para corresponder qualquer valor que não corresponda aos outros padrões. É sempre recomendado usar o "_" para garantir que todos os possíveis valores sejam tratados.



É possível usar também match com estruturas de dados, como structs, enums e tuplas, e destruturando seus campos. Por exemplo:



  match minha_struct {
    MinhaStruct {campo1: valor1, campo2: valor2} => println!("valores correspondem"),
    _ => println!("valores não correspondem"),
}


Além disso, você pode usar "if let" e "while let" para fazer o match em uma expressão e atribuir o resultado a uma variável se o match for bem-sucedido. Por exemplo:



if let MinhaStruct {campo1: valor1, campo2: valor2} = minha_struct { println!("valores correspondem"); } else { println!("valores não correspondem"); }



O "pattern match" é uma ferramenta poderosa em Rust que permite escrever código mais conciso e legível. Ele é usado com frequência em conjunto com enums para tratar casos específicos de uma forma clara e fácil de entender.

*/

