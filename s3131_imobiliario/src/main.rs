

mod imobiliaria;

use imobiliaria::Imobiliaria;

fn main() {
    let mut imobiliaria = Imobiliaria {
        nome: String::from("Imobiliária Guarulhos"),
        endereco: String::from("Rua dos Maias, 95"),
        imoveis: Vec::new(),
    };
    println!("{}",imobiliaria.get_nome());
    println!("{}", imobiliaria.get_endereco());
    println!("Imoveis para Alugar ou Vender");
    imobiliaria.novo_imovel(String::from("Rua Severino, 12"),500000.00,2,1,65.0);
    
    imobiliaria.novo_imovel(String::from("Rua Saviano, 15"),300000.00,6,2,85.0);
    for imovel in imobiliaria.imoveis {
        println!("Endereço: {}  Preço: {} nr. Quartos: {} nr. Banheiros: {}  Metragem: {}",
                 imovel.endereco, imovel.preco, imovel.num_quartos,
                 imovel.num_banheiros, imovel.metragem);
    }
        
    
    
}
/*
pub struct Imobiliaria {
    pub nome: String,
    pub endereco: String,
    pub imoveis: Vec<Imovel>,
}

pub struct Imovel {
    pub endereco: String,
    pub preco: f32,
    pub num_quartos: u8,
    pub num_banheiros: u8,
    pub metragem: f32,
}

impl Imobiliaria {
    pub fn novo_imovel(&mut self, endereco: String, preco: f32, num_quartos: u8, num_banheiros: u8, metragem: f32) {
        let imovel = Imovel {
            endereco,
            preco,
            num_quartos,
            num_banheiros,
            metragem,
        };
        self.imoveis.push(imovel);
    }
    
    pub fn listar_imoveis(&self) {
        for imovel in &self.imoveis {
            println!("Endereço: {} | Preço: {} | Quartos: {} | Banheiros: {} | Metragem: {}",
                     imovel.endereco, imovel.preco, imovel.num_quartos,
                     imovel.num_banheiros, imovel.metragem);
        }
    }
}


mod imobiliaria;
use imobiliaria::Imobiliaria;

fn main() {
    let mut imobiliaria = Imobiliaria {
        nome: String::from("Imobiliária ABC"),
        endereco: String::from("Rua dos Bobos, 123"),
        imoveis: Vec::new(),
    };
    
    imobiliaria.novo_imovel(String::from("Rua dos Bobos, 124"),
                            200_000.00,
                            3,
                            2,
                            150.0);

    imobiliaria.listar_imoveis();
}


*/