
struct Coin{
    value : u32,
    
}


impl Coin{
  fn get_value(&self) -> u32{
     return self.value;

  } 
  pub fn set_value(&mut self, value: u32){
       self.value= value
  }

}


fn main() {
  
    let mut moeda = Coin{value:1};
   
    println!("Valor da moeda  {}", moeda.get_value());

    moeda.set_value(2);
    println!("novo valor da moeda {}", moeda.get_value());
  }

/*
//professor

pub struct Coin {
    pub value: u32
}

impl Coin {
    pub fn new(value: u32) -> Coin {
        Coin { value }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    }
}


//mod coin; se for feito em 2 arquivos coin.rs
//use coin::Coin;
fn main() {
    let mut coin = Coin::new(1);
    println!("Value of coin is {}", coin.get_value());

    coin.set_value(2);
    println!("Value of coin is now {}", coin.get_value());
} */