struct User (String, String, bool, String);
 
 
fn main(){
    let mut pessoa = User (String::from("JoaoPessoa"), String::from("joaoPessoa@gmail"),true, String::from("Homem"));
    pessoa.0 = String::from("joaoPessoa123"); //mut acima para mudar o primeiro dado
    println!("O nome do usuario eh {}, seu email eh {} e seu genero eh {}", pessoa.0, pessoa.1, pessoa.2);
 
    
}

/*
A estrutura User é criada com quatro campos: nome, email, gênero e status.

Na função main, você cria uma instância da struct chamada pessoa, atribuindo valores a cada campo. Em seguida, você usa println! para imprimir o nome, o email e o gênero do usuário.



Leitura Complementar


Tuple Structs em Rust são estruturas de dados que armazenam uma série de valores de diferentes tipos. Elas são semelhantes a tuplas, mas possuem um nome. Isso as diferencia das tuplas convencionais, que são anônimas.

Para criar uma Tuple Struct, você pode usar a seguinte sintaxe:



struct NomeDaStruct(tipo_de_dado_1, tipo_de_dado_2, tipo_de_dado_3);
Por exemplo, aqui está como criar uma Tuple Struct que armazena um nome, idade e endereço:



struct Pessoa(String, i32, String);
Você pode criar uma instância de uma Tuple Struct atribuindo valores aos campos:



let pessoa = Pessoa(String::from("João"), 30, String::from("Rua das Flores, 123"));


Você pode acessar os campos de uma Tuple Struct usando o ponto seguido pelo índice do campo. Por exemplo, para imprimir o nome da pessoa:



println!("Nome: {}", pessoa.0);


Além disso, você também pode desestruturar uma Tuple Struct, atribuindo seus campos a variáveis ​​separadas:



let Pessoa(nome, idade, endereco) = pessoa;


Agora, as variáveis ​​nome, idade e endereco contêm os valores correspondentes dos campos da struct Pessoa.

Em resumo, Tuple Structs em Rust permitem armazenar vários valores de diferentes tipos em uma estrutura de dados com um nome. Elas são semelhantes a tuplas, mas possuem um nome e permitem acesso aos campos usando índices. Além disso, você pode desestruturar uma Tuple Struct atribuindo seus campos a variáveis ​​separadas.



*/