
fn retorna(caractere: &String)->bool{

  let mut guarda='0';
  let mut troca: i32 = 0;
  let x=caractere.len() as i32; 
 
  for n in 0..caractere.len() {  
     match caractere.chars().nth(n) {
      Some(c) => guarda=c,
      None=> {
          println!("não tem index ")
      }  
     }
     for i in 0..caractere.len() { 
    

       if caractere.chars().nth(i)==Some(guarda){
  
        troca=troca +1;
  
       }
    }
   } 
 
  
  if troca > x{
    return true;
  }else{
    return false;
  }
 
 }
  
 



fn main() {
    let caractere = String::from ("lorena");
    if retorna(&caractere){
         println!("existe caracter repetido"); 

    }else {
         println!("caracter são únicos"); 
    } 
    
    
}

/* solução professor
// Função para verificar se uma string possui todos os caracteres únicos
fn tem_caracteres_unicos(input: &str) -> bool {
    let mut conjunto_de_caracteres = [false; 128]; // Assumindo caracteres ASCII
    
    for &c in input.as_bytes() {
        let indice = c as usize;
        if conjunto_de_caracteres[indice] {
            return false; // Caractere já encontrado
        }
        conjunto_de_caracteres[indice] = true;
    }
    
    true // Todos os caracteres são únicos
}

fn main() {
    let string_de_teste = "cateto";
    
    if tem_caracteres_unicos(string_de_teste) {
        println!("A string possui todos os caracteres únicos.");
    } else {
        println!("A string não possui todos os caracteres únicos.");
    }
}

*/