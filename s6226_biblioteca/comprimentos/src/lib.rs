use std::io::{self, BufRead};
use anyhow::{Result, Error};


pub fn cumprimentar_usuario() -> Result<String>{
    println!("Qual seu nome:?");
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer)?;
    if buffer.trim().to_lowercase() != "henrique"{
        return Err(Error::msg("Acesso negado"));  
    }else{
        return Ok(buffer);
    };
}