
struct User{
    username: String,
    email: String,
    ativo: bool,
    genero: String,
}


impl User{
  fn nome_do_usuario(&self){
    println!("O nome do usuario eh {}", self.username);
   }
 
   fn ativo_ou_inativo(&self){
    println!("O nome do usuario eh {}", self.ativo);
   }
}

fn main() {
  
  let pessoa = User {username:String::from("JoaoPessoa"), email:String::from("joaoPessoa@gmail"),ativo:false, genero:String::from("Homem")};

   pessoa.nome_do_usuario();
   pessoa.ativo_ou_inativo();
}
