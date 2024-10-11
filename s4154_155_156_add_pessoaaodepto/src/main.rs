
use std::{collections::HashMap,io};

fn departamento(){
   let mut departamento_pessoas= HashMap::new();
   loop{
       println!("Digite o comando do tipo: Adicione <pessoa> para o <Departamento>");
       let mut comando=String::new();
       io::stdin().read_line(&mut comando).expect("Erro ao ler variável comando"); 
       let comando = comando.trim();
       let mut token_comando = comando.split_whitespace();
       //println!("{:?}",token_comando.nth(1));
      // println!("{:?}",token_comando.nth(1));
       let pessoa = match token_comando.nth(1){
         Some(p) => p,
         None => {
            println!("o comando digitado nao eh valido");
            continue;
           }
         };
         let departamento = match token_comando.nth(1){
            Some(d)=> d,
            None => {
                println!("2. o comando digitado nao eh valido");
                continue;
            }
         };

         let empregado = departamento_pessoas.entry(String::from(departamento)).or_insert(vec![]);
         empregado.push(String::from(pessoa));
         println!("{:?}",departamento_pessoas);
       }

   }





fn main() {
    departamento();
    //println!("Hello, world!");
}
