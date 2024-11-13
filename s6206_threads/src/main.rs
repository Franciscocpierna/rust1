use std::thread;



fn main() {

    let handle1 = thread::spawn(||{
        hello_thread(1);
    });

    let handle2 = thread::spawn(||{
        hello_thread(2);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();  
    /*thread::spawn(||{
     // código executado em uma nova thread
     println!("Nova thread"); 

    });
    // código executado na thread principal // primeiro exemplo de thread
    println!("thread principal");*/
}

fn hello_thread(id: i32){
    println!("olá da thread {} ", id); 
}
