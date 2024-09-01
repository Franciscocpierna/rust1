
use std::io;
fn convert_to_int(data_input: & String)->i32{
    let x=data_input.trim().parse::<i32>().unwrap(); 
    x 
}



fn main() {
    let mut  convertido= String::new();
    println!("Entre com um número:");
    io::stdin().read_line(&mut convertido).expect("erro ao digitar o número");
    
    let  numero= convert_to_int(&convertido);
    println!("A tabuada de {} ",numero);
     for num in 1..11{

        println!("{} X = {}",num,numero*num);
       
     }
    
}
