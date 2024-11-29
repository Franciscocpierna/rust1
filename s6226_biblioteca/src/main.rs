use comprimentos::cumprimentar_usuario;
use anyhow::Error;

fn main() {
    let result: Result<String, Error> = cumprimentar_usuario();
    match result{
        Ok(user_name) => println!("Bem-Vindo {} ",user_name),
        Err(err) => eprintln!("Error {}", err),
    }
}
