use std::io;

fn convert_to_int(data_input: & String)->i32{
    let x=data_input.trim().parse::<i32>().unwrap(); 
    x 
}

/*fn main() {
   let mut number1 = String::new();
   io::stdin().read_line(&mut number1).expect("Erro ao ler number1");
   let mut number2 = String::new();
   io::stdin().read_line(&mut number2).expect("Erro ao ler number2");
   if convert_to_int(&number1) > convert_to_int(&number2) {
           println!("o numero {} é maior que {}",number1,number2);
       }
    else{
      println!("o numero {} é menor ou igual que {}",number1,number2);
}
  
 
   }*/

   fn main() {
    
      /*let mut soma  = 0;
      let mut valor_entrada = String::new();
      io::stdin().read_line(&mut valor_entrada).expect("Erro ao ler valor_entrada");
      let mut valor_i32 = convert_to_int(&valor_entrada);
      while valor_i32 !=0{
         let mut r = valor_i32 %10;
         soma = soma+r;
         valor_i32 = valor_i32/10;
         }
         println!("o valor da soma dos digitos {}", soma); 
         
         */
         //fatorial
      let mut entrada_fatorial = String::new();
      io::stdin().read_line(&mut entrada_fatorial).expect("Erro no valor entrada_fatorial");
      let mut fatorial = 1;
      let mut entrada_int=convert_to_int(&entrada_fatorial);
      while  entrada_int > 1{
         //5*4*3*2 = 120
         fatorial=fatorial*entrada_int;
         entrada_int=entrada_int-1;

      }
      print!("Fatorial = {} ", fatorial);
  

   
   }