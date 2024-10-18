mod viagens;
use viagens::Passageiros;
use viagens::Voo;



fn main() {
    //Crie o programa principal para adicionar alguns passageiros e vôos, bem como exibir os dados.
    let  voo1  = Voo::new_voo(String::from("1"), String::from("São Paulo"), String::from("Sorocaba"), String::from("10/11/2024"), String::from("05:00"));
      
    let passageiro1: Passageiros  = Passageiros::new_passageiro(String::from("Geraldo"), String::from("456BR"), 18);
    let passageiro2: Passageiros = Passageiros::new_passageiro(String::from("Antonia"), String::from("60BR"), 21);
    let passageiro3: Passageiros = Passageiros::new_passageiro(String::from("Joao"), String::from("59BR"), 25);
    Voo::exibir_voos(voo1);
    Passageiros::exibir_passageiros(passageiro1);
    Passageiros::exibir_passageiros(passageiro2);
    Passageiros::exibir_passageiros(passageiro3);
    

    





  
}

  /*
     professor

     mod viagem;
use viagem::{Dados_Passageiros, Dados_Voos, exibir_voos,adicionar_voo, adicionar_passageiro,exibir_passageiros};

// Crie um programa principal
fn main() {
    let mut dados_passageiros: Vec<Dados_Passageiros> = Vec::new();
    let mut dados_voos: Vec<Dados_Voos> = Vec::new();

    // Adicione alguns passageiros
    adicionar_passageiro(&mut dados_passageiros, String::from("João"), String::from("123ABC"), 18);
    adicionar_passageiro(&mut dados_passageiros, String::from("Maria"), String::from("456DEF"), 21);
    adicionar_passageiro(&mut dados_passageiros, String::from("Pedro"), String::from("789GHI"), 25);

    // Adicione alguns vôos
    adicionar_voo(&mut dados_voos, String::from("Voo101"), String::from("São Paulo"), String::from("Rio de Janeiro"), String::from("30/06/2020"), String::from("09:00"));
    adicionar_voo(&mut dados_voos, String::from("Voo102"), String::from("Rio de Janeiro"), String::from("São Paulo"), String::from("01/07/2020"), String::from("09:00"));

    // Exiba os passageiros
    exibir_passageiros(&dados_passageiros);

    // Exiba os vôos
    exibir_voos(&dados_voos);
}
    */