
pub struct Asset {
    pub nome: String,
    pub preco: f64,
    pub quantidade: i32,
}

pub struct Portfolio {
    pub assets: Vec<Asset>,
}
impl Asset{
    pub fn novo_asstet(&mut self,nome: String, preco: f64, quantidade: u32)->Asset {
        Asset{nome,
            preco,
            quantidade}
    }
   pub fn get_nome(&self){
     self.nome
   } 
   pub fn set_nome(&self,nome){
    self.nome=nome
  } 
  pub fn get_preco(&self){
    self.preco
  } 
  pub fn set_preco(&self,preco){
    self.preco=preco
  } 
  pub fn get_quantidade(&self){
    self.quantidade
  } 
  pub fn set_quantidade(&self,quantidade){
    self.quantidade=quantidade
  } 

}
impl Portfolio {
    pub fn new() -> Portfolio {
        Portfolio { assets: vec![] }
    }

    pub fn add_asset(&mut self, asset: Asset) {
        self.assets.push(asset);
    }
    pub fn total_value(&self) -> f64 {
          let soma=0;
          for i in &self.imoveis {
                println!("nome: {} | Preço: {} | Quantidade: {}",
                         i.nome, i.preco, i.quantidade);
                 soma+=i.preco
            }
             soma
        }
    }







/*

pub struct Asset {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

pub struct Portfolio {
    pub assets: Vec<Asset>,
}

impl Portfolio {
    pub fn new() -> Portfolio {
        Portfolio { assets: vec![] }
    }

    pub fn add_asset(&mut self, asset: Asset) {
        self.assets.push(asset);
    }

    pub fn total_value(&self) -> f64 {
        self.assets
            .iter()
            .map(|asset| asset.price * asset.quantity as f64)
            .sum()
    }
}

*/