fn main() {
    let mut x = 5;
    {
        let y = 10;
        println!("O valor de  y eh: {}", y); // y só fica disponível dentro do bloco, x disponível dentro e fora do bloco representado {...} 
    }
    println!("O valor de x eh: {}", x);
    x = 6;
    println!("O novo valor de  x eh: {}", x);
}

/*
Neste exemplo, a variável y é declarada dentro do bloco de código e só pode ser acessada dentro desse bloco. Tentar acessar y fora desse bloco resultaria em um erro de compilação.

Além disso, é importante notar que blocos de código são úteis para controlar a vida útil de recursos alocados dinamicamente, como arquivos e conexões de banco de dados. Isso é conhecido como "gestão de escopo" (scope management). Usando blocos de código, você pode garantir que recursos são liberados assim que não são mais necessários, sem a necessidade de garantir manualmente que a limpeza é feita.



Em Rust, existe algo chamado de "Bloco de inicialização", que é um bloco de código especial usado para inicializar estruturas de dados, como structs e enums. Ele é executado quando uma nova instância da estrutura é criada e pode ser usado para definir valores iniciais para os membros da estrutura, realizar verificações de erro e configurar qualquer outra lógica de inicialização.

Por exemplo:

struct Point {
    x: i32,
    y: i32,
}
 
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
Aqui, o método "new" é um exemplo de um bloco de inicialização para a estrutura Point. Ele é chamado quando um novo ponto é criado, e define os valores iniciais para x e y.

Além disso, é possível usar blocos de código para controlar a fluxo do programa com instruções como if-else, while, for, etc. Cada uma delas tem sua sintaxe e caracteristicas e os blocos de códigos dentro delas são executadas de acordo com as condições especificadas nessas instruções.

Em resumo, os blocos de código são uma ferramenta importante para organizar e controlar o fluxo do código em Rust, permitindo a criação de escopos e inicialização de estruturas de dados, além de garantir a gestão de recursos e a execução de lógica condicional.


*/