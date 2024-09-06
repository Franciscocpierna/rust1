fn main(){
 
    let nome = "hello";
    let nome2 = String::from("hello world");
 
    let hello = &nome2[6..11];
    println!("{}", hello);
    println!("{}", nome);
}

/*
O código acima cria duas variáveis: "nome" e "nome2". "nome" é uma string literal e "nome2" é uma string criada a partir da função "String::from()". Em seguida, é criado um slice chamado "hello" que aponta para a sub-sequência "world" dentro da string "nome2". Então, ele imprime o conteúdo do slice "hello" e o conteúdo da string "nome" no console. Temos como saída:



world
hello
Note que os índices passados para slice são inclusivos no primeiro elemento e exclusivos no último, então &nome2[6..11] pega os caracteres da posição 6 até 10 (inclusive).



Leitura Complementar


Definição:  Um slice é uma referência para uma sub-sequência de uma string. Em Rust, slices são tipos sem possuir propriamente uma cópia dos dados, eles apenas guardam uma referência para os dados originais e os índices inicial e final do slice. Isso faz com que slices sejam mais rápidos e eficientes do que criar uma nova string para cada sub-sequência.

Criando slices: Para criar um slice de uma string, você pode usar o operador "&" seguido dos índices inicial e final da sub-sequência desejada. Por exemplo, se você tem uma string "hello" e deseja criar um slice com as letras "ell", você pode escrever &s[1..3].

Utilizando slices: Slices podem ser usados como qualquer outra string. Você pode usar os métodos de strings comuns, como len() e [] para acessar caracteres específicos. Você também pode atribuir slices a outras variáveis ou passá-las como argumentos para funções.

Slices como argumentos de função: Quando você passa um slice como argumento para uma função, é importante lembrar que a função não possui uma cópia dos dados, ela apenas tem acesso aos dados originais. Isso significa que qualquer alteração feita na função será refletida nos dados originais.

Exemplo:

fn main() {
    let s = "hello";
    let slice = &s[1..3];
    println!("{}", slice); // => "el"
}
Conclusão: Slices são uma forma eficiente de trabalhar com sub-sequências de strings em Rust. Eles permitem acessar e manipular essas sub-sequências sem precisar criar cópias dos dados originais. Lembre-se de sempre ser consciente de como as funções trabalham com slices e como elas afetam os dados originais.



/*