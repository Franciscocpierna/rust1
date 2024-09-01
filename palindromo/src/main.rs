
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
    let numero = "-121";
    let retorno=palindromo(numero);
    if retorno == true{
        println!("É um palindromo");

    }else{
        println!("Não é um palindromo");
    }

}

/*
fn eh_palindromo(x: i32)->boll{
  //casos especiais: Numeros negativos e numeros terminados em 0
  if x<0 || (x % 10 == 0 && x != 0){
      return false;
  } 
  let mut original = x;
  let mut invertido = 0;
  while  original != 0 {
   let digito = % 10;
   invertido = invertido * 10 + digito;
   original /= 10; //foi dividido por 10
     
  } 
  return x==invertido;
}
fn main(){
   let num1 = 121;
   let num2 = -121
   let num3 = 10;
   println("É {} um palindromo? {}", num1, eh_palindromo(num1));
   println("É {} um palindromo? {}", num2, eh_palindromo(num2)); 
   println("É {} um palindromo? {}", num3, eh_palindromo(num3));  
}  
*/

