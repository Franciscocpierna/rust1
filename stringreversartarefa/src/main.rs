fn reverse_string(s: &String)->String{
    let mut inverte = String::new();
    let mut tam= s.len(); 
   // 
    for  (_n,_f) in s.chars().enumerate(){
        tam=tam-1;
        inverte.push(s.chars().nth(tam).unwrap()); 
   
        println!("s.chars().nth(n).unwrap(): {}",s.chars().nth(tam).unwrap());
        println!("valor inverte = {} ",inverte);
        
    }
    inverte
}



fn main() {
    let s: String = String::from ("Hello");
    let retorno=reverse_string(&s);
    println!("original: {} ",s);
    println!("contraria: {} ",retorno)
}


/* solução do professor
fn reverse_string(s: &str) -> String{
    let inverte: String = s.chars().rev().collect(); // rev inverte coleta e atribue
    inverte
}
fn main() {
    let s: String = String::from ("Hello");
    let retorno=reverse_string(&s);
    println!("original: {} ",s);
    println!("contraria: {} ",retorno)
}
*/

