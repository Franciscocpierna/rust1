

fn main() {
    //let mut numeros: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let mut vetores = vec![1,2,3,4];
    //let mut vetores2: vec<i32> = vec::new();
    vetores.push(5);
    println!("{:?}", vetores); 
    vetores.remove(1);
    println!("{:?}", vetores); 
    for i in vetores.iter(){
        println!("{:?}", i);  

    }

}
/*
Este é um código Rust que cria um vetor (semelhante a um array) chamado "vetores" e o inicializa com os valores 1, 2, 3 e 4. Ele então adiciona o valor 5 ao final do vetor usando o método push(). O programa então imprime o valor no quarto índice do vetor (que deve ser 5) usando a macro println!. O programa então remove o valor no índice 1 do vetor usando o método remove().

O programa então imprime todo o vetor usando a macro println! com a flag :?, que diz à macro para imprimir o vetor em um formato mais legível para humanos. Finalmente, o programa usa um loop for para iterar através dos elementos do vetor e imprimir cada um usando a macro println!.



Leitura Complementar


Declarando e Inicializando Vetores

Para declarar um vetor vazio, usamos a sintaxe "let nome_do_vetor: Vec<tipo_de_dado> = Vec::new();". Por exemplo, "let numeros: Vec<i32> = Vec::new();" declara um vetor vazio de inteiros.

Também podemos inicializar um vetor com valores iniciais usando a macro "vec![]". Por exemplo, "let numeros = vec![1, 2, 3, 4];" declara um vetor de inteiros com os valores 1, 2, 3 e 4.



Adicionando elementos a um vetor

Podemos adicionar elementos ao final de um vetor usando o método "push()". Por exemplo, "numeros.push(5);" adiciona o valor 5 ao final do vetor "numeros".



Removendo elementos de um vetor

Podemos remover elementos de um vetor usando o método "remove()". Este método remove o elemento no índice especificado e move todos os elementos à direita para preencher o espaço vago. Por exemplo, "numeros.remove(2);" remove o elemento no índice 2 do vetor "numeros".



Iterando sobre os elementos de um vetor

Podemos iterar sobre os elementos de um vetor usando o método "iter()". Por exemplo, "for numero in numeros.iter() { println!("{}", numero); }" imprime cada elemento do vetor "numeros" em uma nova linha.


*/