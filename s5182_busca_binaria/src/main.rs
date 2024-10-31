

mod search_binary;
use search_binary::busca;

fn main() {
    let numero: Vec<i32> = vec![1, 2, 3, 4, 5];
    let numero1 = 5;
    let resultado: i32 = busca(numero,numero1);

    if resultado == -1{
        println!("Resultado {} não foi achado",numero1);
    }else{
        println!("o resultado é : {}", resultado);
    }

    
   
}

/* professor
mod binary_search;
use binary_search::binary_search;

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 7;
    let result = binary_search(&nums, target);
    if result == -1 {
        println!("O número {} não foi encontrado no vetor.", target);
    } else {
        println!("O número {} foi encontrado no índice {} do vetor.", target, result);
    }
}

Com essa implementação, a função binary_search recebe um vetor de inteiros e um inteiro como argumentos, e retorna o índice do elemento procurado no vetor, ou -1 caso ele não seja encontrado. A função usa um laço while para dividir o vetor em uma seção cada vez menor, comparando o valor no meio desse subvetor com o valor procurado. Se o valor for menor que o procurado, a busca é continuada apenas na metade superior do vetor, caso contrário, é continuada apenas na metade inferior. Esse processo é repetido até que o elemento seja encontrado ou até que não haja mais elementos para buscar.


*/