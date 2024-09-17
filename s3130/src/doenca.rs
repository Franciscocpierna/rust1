pub struct Resfriado {
   pub classifica: String,
   pub duracao: String ,
}


pub trait Duracaot {
   fn leve(&self) -> String;
   fn medio(&self) -> String;
   fn grave(&self)-> String;
}

impl Duracaot for Resfriado {
   fn leve(&self) -> String {
       self.duracao.clone() + " " +&self.classifica
   }

   fn medio(&self) -> String {
       self.duracao.clone() +" "+ &self.classifica
   }

   fn grave(&self) -> String{
      self.duracao.clone()+" " + &self.classifica
   }

   
}