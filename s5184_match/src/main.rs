use std::any::{Any,TypeId};

fn trata_entrada(valor: &dyn Any) {
    match valor.type_id() {
        // Tratando números inteiros
        id if id==TypeId::of::<i32>() => {
           if let Some(valor_int) = valor.downcast_ref::<i32>(){
            println!("O dobro do número inteiro é: {}", valor_int * 2);
        }},
        // Tratando strings
        id if id==TypeId::of::<String>() => {
            if let Some(valor_string) = valor.downcast_ref::<String>(){
             println!("O tamanho da String: {}", valor_string.len());
         }},
        // Tratando vetores
        id if id==TypeId::of::<Vec<i32>>() => {
            if let Some(valor_vec) = valor.downcast_ref::<Vec<i32>>(){
             println!("O tamanho do vetor: {}", valor_vec.len());
         }},
        _ => println!("Tipo de entrada não suportado"),
    }
}

fn main() {
    trata_entrada(&5);
    trata_entrada(&"hello world".to_string());
    trata_entrada(&vec![1, 2, 3]);
}







/*
Nesta questão, você será desafiado a escrever uma função em Rust que utiliza o conceito de pattern matching para tratar diferentes tipos de entradas. A função deve receber um valor como parâmetro e, de acordo com o tipo do valor, deve realizar uma ação específica. Por exemplo, se o valor for um número inteiro, a função deve imprimir o valor dobro; se for uma string, deve imprimir o tamanho da string; se for um vetor, deve imprimir o tamanho do vetor.


A função "trata_entrada" recebe um valor do tipo "&dyn Any" como parâmetro. Dentro da função, é utilizado o conceito de pattern matching para verificar o tipo do valor e realizar a ação específica de acordo com o tipo. O match é usado para comparar o tipo_id do valor com o tipo esperado usando a função type_id::<T>() onde T é o tipo esperado, e fazemos o downcasting para o tipo esperado para realizar a ação. Na função main, são passados diferentes tipos de valores para a função "trata_entrada" para testar o funcionamento do código.

A importação "std::any::Any" é usada para permitir que uma função ou estrutura possa aceitar qualquer tipo de valor como parâmetro, sem que seja necessário especificar o tipo específico. Isso é possível graças ao conceito de "type erasure" que é implementado pela trait Any.

Com o uso de Any, é possível passar qualquer tipo de valor para a função, e usar pattern matching para determinar qual é o tipo específico desse valor, e realizar a ação desejada. Isso permite que a função "trata_entrada" seja mais flexível e possa ser reutilizada em diferentes contextos, sem precisar ser modificada para suportar novos tipos de valores.

A trait Any também fornece métodos para realizar o downcasting de um valor para um tipo específico, permitindo que a função "trata_entrada" possa acessar e utilizar os métodos do valor passado como parâmetro, independentemente do tipo específico.



type_id(): Este método é usado para obter o tipo_id de um valor. Ele é chamado no objeto Any, e retorna uma estrutura TypeId, que pode ser comparada com o tipo esperado usando a função type_id::<T>() onde T é o tipo esperado, para determinar se o valor passado é do tipo esperado.

downcast_ref<T>(): Este método é usado para realizar o downcasting de um valor de &dyn Any para um tipo específico T. Ele retorna um Option<&T>, então é necessário usar o unwrap() para acessar o valor específico.

type_id::<T>(): É usado para obter o TypeId de um tipo específico T, para comparar com o TypeId obtido através do método type_id().

unwrap(): Este método é usado para acessar o valor armazenado dentro de um Option, retornando o valor contido nele. No caso do downcasting, este método é usado para acessar o valor específico depois de ter sido convertido.

O parâmetro passado para a função trata_entrada é um tipo &dyn Any, é uma referência dinâmica para qualquer tipo de dados.

Em resumo, o código está comparando o tipo_id do valor passado com o tipo esperado usando o type_id::<T>(), e fazendo o downcasting para o tipo esperado para realizar a ação desejada.


*/