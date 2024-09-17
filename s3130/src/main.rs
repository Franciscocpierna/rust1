

mod doenca;
use doenca::{Duracaot, Resfriado};


fn main(){
 

    
    let res = Resfriado {
         classifica: String::from("grave"), 
         duracao: String::from("Quarenta Dias") 
        };

    
    if res.classifica =="leve"{
       println!("o refriado: {}", res.leve());
    }else if res.classifica =="medio" {
       println!("o refriado: {}", res.medio()); 
    }else {
       println!("o refriado: {}", res.grave()); 
    } 
    
   
}

/*
use std::fmt;

pub struct Doenca {
    pub nome: String,
    pub sintomas: Vec<String>,
    pub causa: String,
    pub tratamento: String,
}

impl Doenca {
    pub fn new(nome: String, sintomas: Vec<String>, causa: String, tratamento: String) -> Doenca {
        Doenca {
            nome,
            sintomas,
            causa,
            tratamento,
        }
    }
    
    pub fn get_nome(&self) -> &String {
        &self.nome
    }
    
    pub fn get_sintomas(&self) -> &Vec<String> {
        &self.sintomas
    }
    
    pub fn get_causa(&self) -> &String {
        &self.causa
    }
    
    pub fn get_tratamento(&self) -> &String {
        &self.tratamento
    }
}

impl fmt::Display for Doenca {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nome: {}\nSintomas: {:?}\nCausa: {}\nTratamento: {}\n", 
            self.nome, self.sintomas, self.causa, self.tratamento)
    }
}

 mod doenca;

use doenca::Doenca;

fn main() {
    let gripe = Doenca::new(String::from("Gripe"), vec![String::from("Tosse"),String::from("Febre")],
        String::from("Vírus"), String::from("Repouso, medicamentos"));
    
    println!("{}", gripe);
}

*/
    
