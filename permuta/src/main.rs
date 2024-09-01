fn verpermuta(primeira: &String, segunda: &String)->bool{
  let mut guarda='0';
  let mut troca: i32 = 0;
  let x=primeira.len() as i32;   
  if primeira.len()> segunda.len(){
       return false;  
  }else {
    if segunda.len()> primeira.len(){
          return false
    }
  }
  for n in 0..primeira.len(){
    match primeira.chars().nth(n) {
        Some(c) => guarda=c,
        None=> {
            println!("não tem index ")
        }  
    }
    
    for i in 0..primeira.len() { 
       
       if primeira.chars().nth(i)==Some(guarda) && segunda.chars().nth(i)==Some(guarda) {
            return false; // retornou o caracter em mesma  posição não indica permuta
            
       
            }

       if primeira.chars().nth(i)==Some(guarda) || segunda.chars().nth(i)==Some(guarda) {
        
                 troca=troca +1;
           
                }
         }
    
        }
        
        if troca ==2*x{
            return true;
          }else{
            return false;
          }
}


fn main() {
    let primeira = String::from ("bec");
    let segunda = String::from ("cbe");
    if verpermuta(&primeira,&segunda){
         println!("é permutação"); 

    }else {
         println!("não é permutação"); 
    } 
    
}
/*
solução do professor

// Função para verificar se uma string é uma permutação de outra
fn eh_permutacao(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false; // Diferentes comprimentos, não podem ser permutações
    }

    let mut contagem_caracteres = [0; 128]; // Assumindo caracteres ASCII

    // Conta as ocorrências de caracteres na primeira string
    for &c in str1.as_bytes() {
        contagem_caracteres[c as usize] += 1;
    }

    // Decrementa as ocorrências de caracteres com base na segunda string
    for &c in str2.as_bytes() {
        contagem_caracteres[c as usize] -= 1;
        if contagem_caracteres[c as usize] < 0 {
            return false; // Mais ocorrências do caractere na segunda string
        }
    }

    true // Todos os caracteres na primeira string têm ocorrências correspondentes na segunda string
}

fn main() {
    let str1 = "abc";
    let str2 = "bca";

    if eh_permutacao(str1, str2) {
        println!("As strings são permutações uma da outra.");
    } else {
        println!("As strings não são permutações uma da outra.");
    }
}

*/