fn main(){
    // let numeros=[2,1000] //preenche array com 2 mil vezes
    let numeros_inteiros: [i32; 5] = [1, 2, 3, 4, 5];
    let  j = numeros_inteiros.len();
    println!("{}numeros_inteiros[j]",numeros_inteiros[j-1]);
    println!("{} tamanho do vetor = ", j);
    println!("----------------------");
    for n in numeros_inteiros.iter(){
        println!("{}", n);
    }
 
    
}
/*
Este é um programa simples em Rust que define um array de inteiros chamado "numeros_inteiros" com os valores [1, 2, 3, 4, 5]. Em seguida, usa um loop "for" para percorrer o array e imprimir cada valor usando a macro "println!". O programa irá exibir os números de 1 a 5, cada um em uma nova linha.



Leitura Complementar


O que é um array?



Um array é uma estrutura de dados que armazena uma coleção de elementos do mesmo tipo. Em Rust, os arrays são estáticos, o que significa que seu tamanho é fixo e não pode ser alterado depois de criado.

Como declarar arrays em Rust?

Para declarar um array em Rust, usamos a seguinte sintaxe:

let nome_array: [tipo; tamanho] = [valor1, valor2, valor3, etc.];
Exemplo:

let numeros: [i32; 5] = [1, 2, 3, 4, 5];
Neste exemplo, estamos criando um array chamado "numeros" que armazena 5 elementos do tipo i32 (inteiro de 32 bits) com os valores [1, 2, 3, 4, 5].



Acessando elementos de um array

Para acessar um elemento de um array, usamos o nome do array seguido de colchetes com o índice do elemento desejado. Lembre-se que os índices começam em zero.

Exemplo:

let primeiro_numero = numeros[0];
Neste exemplo, estamos atribuindo o valor armazenado na posição 0 do array "numeros" à variável "primeiro_numero".



Iterando sobre arrays

Podemos usar um loop "for" para iterar sobre os elementos de um array. A sintaxe é similar ao usado com outros tipos de coleções, como strings e listas.

Exemplo:

for n in numeros.iter(){
    println!("{}", n);
}
Neste exemplo, estamos usando o método "iter" do array "numeros" para obter um iterador dos elementos do array. Em seguida, usamos um loop "for" para percorrer os elementos e imprimir cada um usando a macro "println!".



Outras operações comuns

O método "len" retorna o tamanho do array

O método "is_empty" retorna se o array está vazio ou não

O método "sort" ordena os elementos do array

O método "reverse" inverte a ordem dos elementos do array.

Em resumo, os arrays são uma estrutura de dados importante em Rust, que nos permite armazenar e acessar uma coleção de elementos do mesmo tipo de maneira eficiente. Através dos exemplos apresentados, você aprendeu como declarar e acessar arrays, como iterar sobre os elementos e algumas outras operações





*/