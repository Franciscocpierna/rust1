//campos endereco, preco, num_quartos, num_banheiros e metragem.
// novo_imovel(endereco, preco, num_quartos, num_banheiros e metragem.) 
// objeto do tipo Imovel 
//use std::fmt;

pub struct Imobiliaria {
    pub nome: String, 
    pub endereco: String,
    pub imoveis: Vec<Imoveis>,
}
pub struct Imoveis{    
    pub endereco: String,
    pub preco: f64,                   
    pub num_quartos: u32,
    pub num_banheiros: u32,
    pub metragem: f64,
}


    


impl Imobiliaria {
    pub fn novo_imovel(&mut self,endereco: String, preco: f64, num_quartos: u32, num_banheiros: u32, metragem: f64) {

        let imoveis = Imoveis {endereco,preco,num_quartos,num_banheiros,metragem};
        self.imoveis.push(imoveis);
         
    }
        
    pub fn get_nome(&self) -> &String {
        &self.nome
    }
    
    pub fn get_endereco(&self) -> &String {
        &self.endereco
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

*/