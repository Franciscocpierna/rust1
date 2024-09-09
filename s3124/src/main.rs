extern crate rand;
use rand::Rng;
 
 
fn main(){
    let mut x = 1;
    loop {
     let valores_randomicos = rand::thread_rng().gen_range(0, 101);
     println!("{}", valores_randomicos);
     if x ==10{
        break;
     }
     x+=1;

    } 
    
 
 
}
 /*
 professor
extern crate rand;
use rand::Rng;

fn gera_aleatorios() {
    for _ in 0..10 {
        println!("{}", rand::thread_rng().gen_range(0, 101));
    }
}

fn main() {
    gera_aleatorios();
}


 */