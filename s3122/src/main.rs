extern crate rand;
use rand::Rng;
 
 
fn main(){
    let mut  x=0; 
    while x !=10{
     let valores_randomicos = rand::thread_rng().gen_range(5, 11);
     println!("{}", valores_randomicos);
     x+=1;
    }
 
}
