mod finance;
use finance::Asset;
use finance::Portfolio;
//use finance::{Asset, Portfolio};


fn main() {
    let novoassept=finance::Asset::novo_asset("Ativo".to_string(),2300.00,5);
    let novoassept1=finance::Asset::novo_asset("Ativo1".to_string(),500.00,10);
    let mut portfolio = finance::Portfolio::new();

    portfolio.add_asset(novoassept);
    portfolio.add_asset(novoassept1);
    
   
    let soma=0.0;
    println!("Valor total do portfólio: ${:.2}", portfolio.total_value(soma));
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

mod finance;
use finance::{Asset, Portfolio};

fn main() {
    let mut portfolio = finance::Portfolio::new();

    portfolio.add_asset(finance::Asset {
        name: "AAPL".to_string(),
        price: 135.0,
        quantity: 5,
    });

    portfolio.add_asset(finance::Asset {
        name: "GOOG".to_string(),
        price: 1235.0,
        quantity: 2,
    });

    println!("Valor total do portfólio: ${:.2}", portfolio.total_value());
}
*/