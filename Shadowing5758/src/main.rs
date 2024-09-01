


/*

Shadowing é uma técnica de nomeação de variáveis em Rust que permite que uma variável seja redeclarada com o mesmo nome em um escopo mais interno. Quando isso acontece, a variável mais interna "sombreia" (shadow) a variável mais externa, tornando-a temporariamente inacessível. Aqui está um exemplo de como o shadowing é usado em Rust:
*/
fn main() {
   let x = 5; // x é uma variável de nível de escopo com o valor 5
 
    {
     let x = x + 1; // x é sombreado aqui com o valor 6 e isso é Shadowing 
     println!("x dentro do bloco: {}", x); // imprime "x inside: 6"
    }
 
    println!("x saída do bloco volta valor fora 5: {}", x); // imprime "x outside: 5"
}

/*
Aqui, a variável x é declarada inicialmente com o valor 5. Em seguida, uma nova variável x é declarada dentro de um escopo de bloco, usando o valor da variável x anterior mais 1. Isso "sombreia" a variável x original, fazendo com que o seu valor de 5 seja temporariamente inacessível. Dentro desse escopo de bloco, a variável x tem o valor 6, e ao sair desse escopo, acessamos novamente o valor original de x = 5

Além de redeclarar variáveis, o shadowing também pode ser usado para mudar o tipo de uma variável. Isso é útil quando você precisa reutilizar o nome de uma variável, mas precisa dar a ela um tipo diferente.



let x: i32 = 5;
let x = x as f64;
Aqui a variável x é declarada como i32 e depois é "sombreada" com um novo tipo f64.

É importante mencionar que em rust toda atribuição é por valor, então quando uma variavel é sombreada, é criado uma nova variavel e o valor da original é copiado para a nova variavel sombreada.

É possível também "sombrear" constantes, utilizando a keyword const ao invés de let, seguindo as mesmas regras e comportamentos.

Shadowing pode ser uma técnica útil quando você precisa reutilizar o nome de uma variável, mas precisa dar a ela um novo valor ou tipo. É importante ter cuidado ao usá-lo, pois ele pode tornar o código mais difícil de entender se usado de forma inadequada.
*/