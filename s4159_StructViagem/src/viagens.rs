


//Crie uma struct para armazenar os dados dos passageiros.
pub struct Passageiros{
   pub nome:String,
   pub numero_passaporte: String,
   pub idade: u8
}

//2. Crie uma struct para armazenar os dados dos vôos.
pub struct Voo{
   pub codigo_voo: String,
   pub partida: String,
   pub destino: String,
   pub data_partida: String,
   pub hora_partida: String
}
 impl Passageiros{
  // Crie uma função para adicionar um novo passageiro.
  pub fn new_passageiro(nome: String,numero_passaporte: String,idade: u8) ->Passageiros{
        Passageiros {
            nome,
            numero_passaporte,
            idade

        }
  }
  // Crie uma função para exibir todos os passageiros.
  pub fn exibir_passageiros(passageiro: Passageiros){
     println!("Exibindo passageiros");
     println!("Nome: {}", passageiro.nome);
     println!("Passaporte: {}", passageiro.numero_passaporte);
     println!("Idade: {}", passageiro.idade);
     println!();
  }
 }

 impl Voo{
   //Crie uma função para adicionar um novo vôo.
   pub fn new_voo(codigo_voo: String,partida: String,destino: String,data_partida: String,hora_partida: String) ->Voo{
     Voo{
        codigo_voo,
        partida,
        destino,
        data_partida,
        hora_partida
     }

    }

    //Crie uma função para exibir todos os vôos.
   pub fn exibir_voos(voo: Voo){
       println!("Exibindo Voo");
       println!("Codigo: {}", voo.codigo_voo);
       println!("Partida: {}", voo.partida);
       println!("Destino: {}", voo.destino);
       println!("Data da Partida: {}", voo.data_partida);
       println!("Hora da Partida: {}", voo.hora_partida);
       println!();
      
   }
}

   











/* professor
// Crie uma struct para armazenar os dados dos passageiros
pub struct Dados_Passageiros {
    pub nome: String,
    pub numero_passaporte: String,
    pub idade: u8
}

// Crie uma struct para armazenar os dados dos vôos
pub struct Dados_Voos {
    pub codigo_voo: String,
    pub partida: String,
    pub destino: String,
    pub data_partida: String,
    pub hora_partida: String
}

// Crie uma função para adicionar um novo passageiro
pub fn adicionar_passageiro(dados_passageiros: &mut Vec<Dados_Passageiros>, nome: String, numero_passaporte: String, idade: u8) {
    let passageiro = Dados_Passageiros { nome, numero_passaporte, idade };
    dados_passageiros.push(passageiro);
}

// Crie uma função para adicionar um novo vôo
pub fn adicionar_voo(dados_voos: &mut Vec<Dados_Voos>, codigo_voo: String, partida: String, destino: String, data_partida: String, hora_partida: String) {
    let voo = Dados_Voos { codigo_voo, partida, destino, data_partida, hora_partida };
    dados_voos.push(voo);
}

// Crie uma função para exibir todos os vôos
pub fn exibir_voos(dados_voos: &Vec<Dados_Voos>) {
    for voo in dados_voos {
        println!("Código de Vôo: {}", voo.codigo_voo);
        println!("Partida: {}", voo.partida);
        println!("Destino: {}", voo.destino);
        println!("Data de Partida: {}", voo.data_partida);
        println!("Hora de Partida: {}\n", voo.hora_partida);
    }
}

// Crie uma função para exibir todos os passageiros
pub fn exibir_passageiros(dados_passageiros: &Vec<Dados_Passageiros>) {
    for passageiro in dados_passageiros {
        println!("Nome: {}", passageiro.nome);
        println!("Numero de Passaporte: {}", passageiro.numero_passaporte);
        println!("Idade: {}\n", passageiro.idade);
    }
}
*/
