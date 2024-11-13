use std::thread;

fn main() {

    let result = thread::spawn(||{
        Err::<&str, _>("Algo deu errado")
    }).join().unwrap();
    match result{
        Ok(value) => println!("Thread bem sucedida com valor {:?}", value),
        Err(err) =>  println!("Erro da thread {:?}", err),

    }
}
