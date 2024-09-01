struct User{
    username: String,
    email: String,
    ativo: bool,
    genero: String,
}
 
fn main(){
    let mut pessoa = User {username:String::from("JoaoPessoa"), email:String::from("joaoPessoa@gmail"),ativo:true, genero:String::from("Homem")};
    pessoa.ativo=false;
    println!("O nome do usuario eh {}, seu email eh {} e seu genero eh {}", pessoa.username, pessoa.email, pessoa.genero);
    println!("O nome do usuario Ativo {}",pessoa.ativo);
}