use std::thread;



fn main() {
    thread::spawn(||{
     // código executado em uma nova thread
     println!("Nova thread"); 

    });
    // código executado na thread principal
    println!("thread principal");
}
