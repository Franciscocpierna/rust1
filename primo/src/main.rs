use std::io;
fn convert_to_int(data_input: & String)->i32{
    let x=data_input.trim().parse::<i32>().unwrap(); 
    x 
}
fn primo(numero: &i32)->bool{
 let mut x = *numero;  
 let mut divisao = 0;
 
 
 while x != 0 {
  if *numero % x ==0{
      divisao = divisao +1;    
   }

   x= x-1;
 }
println!("numero divisão: {} ",divisao);
 if divisao == 2{
    return true;
     
  }else{
    return false;
  }

}





fn main() {
    let mut  convertido= String::new();
    println!("Entre com um número:");
    io::stdin().read_line(&mut convertido).expect("erro ao digitar o número");
    
    let mut numero= convert_to_int(&convertido);
    if primo(&numero)==true{
        println!("este número é primo {}", numero)
    }else{
        println!("este número não é primo")
    }



}
