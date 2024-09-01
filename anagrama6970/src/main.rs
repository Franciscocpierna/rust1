fn is_anagram(s: &str,t: &str)->bool{
    let mut inverte: String = s.chars().rev().collect();
    if inverte == t{
        return true;
    }
    if s.len() != t.len(){
        return false;
    }
    if s==t{
        return false;
    }
    let mut tam= s.len(); 
    let mut ocorrencia=0;
    
    

  
    
    for  (n,f) in s.chars().enumerate(){
        if tam != s.len(){
          if ocorrencia==0{
              return false;
          }else{
            ocorrencia=0;
          }
        }
        for  (g,h) in t.chars().enumerate(){
         if s.chars().nth(n).unwrap()==t.chars().nth(g).unwrap(){
            ocorrencia =1;
         }           
   
        
        }
        tam=tam-1;
    }
    return true;

}


fn main() {
    let  s1="anagram";
    let  t1="nagaram";
    println!("{}", is_anagram(s1,t1));
    let  s2="tar";
    let  t2="car";
    
    println!("{}", is_anagram(&s2,t2));
} 

//exemplo professor
/*fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    println!("vect na  posicao 1 {}",s_chars[1]);
    
    // Sort both vectors of characters classifica ambos vetores caracteres
    s_chars.sort();
    t_chars.sort();

    // Compare if the sorted vectors are equal
    s_chars == t_chars
}



fn main() {
    let s1 = "anagram";
    let t1 = "nagaram";
    println!("{}", is_anagram(s1, t1)); // Output: true

    let s2 = "rat";
    let t2 = "car";
    println!("{}", is_anagram(s2, t2)); // Output: false
} */