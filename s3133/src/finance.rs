
pub struct Asset {
    pub nome: String,
    pub preco: f64,
    pub quantidade: i32,
}

pub struct Portfolio {
    pub assets: Vec<Asset>,
}
impl Asset{
    pub fn novo_asset(nome: String, preco: f64, quantidade: i32)->Asset {
        Asset{nome,
            preco,
            quantidade}
    }
    pub fn get_nome(&self) -> &String {
        &self.nome
    }
   pub fn set_nome(&mut self,nome:String){
      self.nome=nome
  } 
  
  pub fn get_preco(&self)->f64{
    self.preco
  } 
  pub fn set_preco(&mut self,preco:f64){
    self.preco=preco
  } 
  pub fn get_quantidade(&self)->i32{
    self.quantidade
  } 
  pub fn set_quantidade(&mut self,quantidade:i32){
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
    pub fn total_value(&self,mut soma:f64) -> f64 {
          
          for i in &self.assets {
                println!("nome: {} | Preço: {} | Quantidade: {}",
                         i.nome, i.preco, i.quantidade);
                 soma=soma+i.preco*i.quantidade as f64;
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