fn compress_string(original_str: &str)->String{
   
   let mut original_modificada = String::new();
   let mut soma = 0;
   let mut guarda='0';
   let mut guardaanterior='0'; 
   let total= original_str.len();
   match original_str.chars().nth(0) {
    Some(c) => guarda=c,
    None=> {
     println!("não tem index ")
 
    }
   }
   guardaanterior=guarda;
   for n in 0..total{ 
     
       if original_str.chars().nth(n)==Some(guarda){
            soma = soma +1;
       }
        
      
       if original_str.chars().nth(n)!=Some(guarda){
         if soma > 1{
            
            original_modificada.push_str(&soma.to_string());
            original_modificada.push(guardaanterior);
            
            match original_str.chars().nth(n) {
                Some(c) => guarda=c,
                None=> {
                 println!("não tem index ")
             
                }      
              }
              guardaanterior=guarda;  
         }else{
            original_modificada.push(guardaanterior);
            match original_str.chars().nth(n) {
                Some(c) => guarda=c,
                None=> {
                 println!("não tem index ")
             
                }   
              }  
            }      
         guardaanterior=guarda;
         soma=1;
         }
         
         
       }
     
       if soma > 1{
        original_modificada.push_str(&soma.to_string());
        original_modificada.push(guardaanterior);

       }else {
        original_modificada.push(guardaanterior);
       }     
     
    
     return original_modificada;
        }
    





fn main() {
    let original_str = "aaabccccaaaa";
    let compressed_str = compress_string(original_str);
    println!("Original: {}", original_str);
    println!("Compressed: {}", compressed_str);

    let other_str = "abcdefgh";
    let compressed_other = compress_string(other_str);
    println!("Original: {}", other_str);
    println!("Compressed: {}", compressed_other);
}


/*
fn compress_string(s: &str)->String{
  let mut compressed = String::new();
  let mut count=0; 
 // percorre a string para realizar a compressão
  for (i,c) in s.chars().enumerate(){
   count +=1;
   //verificar se o pçroximo caracter é diferente ou chegamos ao final da string
   if i +1 >= s.len()||c!=s.chars().nth(i+1).unwrap(){
        compressed.push(c);
        compressed.push_str(&count.to_string());
        count=0;// reinicia a contagem para o proximo cartacter
   }
  }
    if compressed.len()>=s.len(){
        s.to_string();//ou usa return
    }else{
      compressed // ou usa return
    }

}
fn main() {
    let original_str = "aaabccccaaaa";
    let compressed_str = compress_string(original_str);
    println!("Original: {}", original_str);
    println!("Compressed: {}", compressed_str);

    let other_str = "abcdefgh";
    let compressed_other = compress_string(other_str);
    println!("Original: {}", other_str);
    println!("Compressed: {}", compressed_other);
}
*/