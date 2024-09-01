fn palindromo(numero: &str)->bool{
    let mut inverte = String::new();
    let mut tam= numero.len(); 
   // 
    for  (n,f) in numero.chars().enumerate(){
        tam=tam-1;
        inverte.push(numero.chars().nth(tam).unwrap()); 
   
        println!("numero.chars().nth(n).unwrap(): {}",numero.chars().nth(tam).unwrap());
        println!("valor inverte = {} ",inverte);
        
    }
    if inverte == numero{
       return true;
    }else{
        false
    }

}


fn main() {
    let numero = "a1a";
    let retorno=palindromo(numero);
    if retorno == true{
        println!("É um palindromo");

    }else{
        println!("Não é um palindromo");
    }

}

/*exemplo do professor

fn eh_palindromo(frase: &str) -> bool {
    let frase_limpa = frase.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "");
    let tamanho = frase_limpa.len();
    for i in 0..tamanho/2 {
        if frase_limpa.chars().nth(i) != frase_limpa.chars().nth(tamanho-i-1) {
            return false;
        }
    }
    true
}

fn main() {
    let frase = "A man, a plan, a canal: Panama";
    let resultado = eh_palindromo(frase);
    println!("A frase '{}' é um palíndromo? {}", frase, resultado);
}


*/