struct User{
    username: String,
    email: String,
    ativo: bool,
    genero: String,
}
 
fn user(usuario: &User){
    println!("O nome do usuario eh {}", usuario.username);
}
 
fn main(){
    let mut pessoa = User {username:String::from("JoaoPessoa"), email:String::from("joaoPessoa@gmail"),ativo:true, genero:String::from("Homem")};
    pessoa.ativo = false;
    println!("O nome do usuario eh {}, seu email eh {} e seu genero eh {}", pessoa.username, pessoa.email, pessoa.genero);
    user(&pessoa);
    user(&pessoa);
    
}


/*
Esse código cria uma struct chamada "User" que possui quatro campos: "username" (do tipo String), "email" (do tipo String), "ativo" (do tipo bool) e "genero" (do tipo String). Ele também possui uma função "user" que recebe uma referência para um objeto "User" e imprime o nome de usuário.

Na função main(), cria-se uma variável "pessoa" do tipo "User" com valores iniciais específicos para cada campo e é feita uma modificação na variavel ativo. Em seguida, o código imprime algumas informações sobre a pessoa, usando a interpolação de string, e chama a função "user" duas vezes, passando a referência para "pessoa" como argumento. Isso resultaria em imprimir o nome de usuário duas vezes.

Leitura Complementar
Passagem por referência em Rust é quando uma função recebe uma referência de um objeto em vez de uma cópia desse objeto. Isso permite que a função altere o objeto original.

Para passar um objeto por referência, você deve usar o operador "&" antes do nome do objeto. Por exemplo:



fn increment(x: &mut i32) {
    *x += 1;
}
 
let mut number = 5;
increment(&mut number);
println!("The value of number is: {}", number);


Aqui, a função increment recebe uma referência mutável para o objeto number. A função então incrementa o valor de number usando o operador de desreferenciação "*". Depois disso, o valor de number é impresso como 6.

É importante notar que ao passar um objeto por referência, você deve garantir que ele não será perdido. Isso pode ser feito usando "empréstimo" (borrow). Em Rust, você pode emprestar um objeto para uma função, garantindo que ele não será perdido durante a execução da função.

Aqui está um exemplo de empréstimo:



fn print_value(x: &i32) {
    println!("The value is: {}", x);
}
 
let number = 5;
print_value(&number);


Aqui, a função print_value empresta o objeto number sem modificá-lo. Isso permite que o objeto original seja usado após a chamada da função.

Em resumo, a passagem por referência permite que uma função altere o objeto original e o empréstimo garante que o objeto não será perdido durante a execução da função.



*/