
//referencia

fn main() {
    /*let x=10;
    let y = &x; // sinal & é refencia a variavel x
    let z = &x;
    println!("valor de x e y e z {} {} {}", x,y,z);*/ //funcionou
    let mut x=10;
    let y = &mut x; // sinal & é refencia a variavel x
    *y+=1; // para fazer este emprestimo / referencia *y acrescentar mut acima para atribuir a y
    //let z = &x;
    println!("valor de x e y e z {}  ", y); 
}

/*
Revisão References
Em Rust, as referências são usadas para dar acesso aos dados sem transferir a propriedade dos mesmos. Isso é útil quando você deseja passar um grande conjunto de dados para uma função sem copiá-lo, ou quando deseja ter vários ponteiros para o mesmo dado.

Aqui está um exemplo de como usar referências em Rust:

let x = 5;
let y = &x;
Aqui, x é uma variável inteira com o valor 5 e y é uma referência para x. A referência é criada usando o operador &, que é chamado de "operador de referência".

Ao acessar o valor de uma referência, usamos o dereferencing (operador *), no caso, *y retorna o valor 5.



let x = 5;
let y = &x;
 
println!("x is {}", x); 
println!("y is {}", *y);
É importante notar que as referências não podem ser mudadas diretamente, é necessário usar &mut para obter uma referência mutável

let mut x = 5;
let y = &mut x;
*y += 1;
println!("x is {}", x); 
Outro conceito importante em rust são as lifetimes, são regras que garantem a segurança de uso de referências, elas garantem que uma referência nunca vai se referir a um dado que foi descartado.

fn do_something(x: &i32) {
    println!("{}", x);
}
 
let y = &5;
do_something(y);
Neste caso acima, o lifetime do y foi definido automaticamente como 'static, isso significa que ele ira viver durante toda a vida do programa, se você deseja trabalhar com lifetime de forma mais especifica é possível fazer isso de forma explicita

fn do_something<'a>(x: &'a i32) {
    println!("{}", x);
}
Em resumo, as referências em Rust permitem acessar dados sem transferir a propriedade dos mesmos e são essenciais para trabalhar com dados de grande porte e múltiplas referências ao mesmo dado. O uso das referências em Rust é garantido pelo sistema de lifetime, que garante que as referências não apontem para dados descartados e fornece um mecanismo para garantir a segurança e correção do uso das mesmas.



Conteúdo do curso
Reproduzir
43. Tuplas

*/
