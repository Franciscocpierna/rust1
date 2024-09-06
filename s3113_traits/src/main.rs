struct Pessoa{
    nome: String,
    idade: i32
}
 
 
trait Voz{
    fn falar(&self);
 
    fn tem_voz(&self) -> bool;
}
 
impl Voz for Pessoa{
    fn falar(&self){
        println!("Ola, meu nome eh {}", self.nome);
    }
 
    fn tem_voz(&self) -> bool{
        if self.idade > 1{
            return true;
        }
        return false;
    }
}
 
fn main(){
 
    let pessoa = Pessoa{
        nome: String::from("Joao"),
        idade: 42,
    };
 
    println!("Pode {} falar ? {}", pessoa.nome, pessoa.tem_voz() );
    
 
    
}

