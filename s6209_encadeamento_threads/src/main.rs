use std::thread;

fn main() {

    let result = thread::spawn(||{
        42
    }).join().unwrap();
    
    let result2 = thread::spawn(move ||{  //move usado para usar na thread variaveis externa result movida
        result*2
    }).join().unwrap();

    

    println!("resultado final {}", result2);
    
}
