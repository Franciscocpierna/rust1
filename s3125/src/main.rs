
use rand::Rng;
use std::io;

fn convert_to_int(data_input: & String)->i32{
    let x=data_input.trim().parse::<i32>().unwrap(); 
    x 
}

fn main() {
    //println!("Adivinhe o número!");

    let numero = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Qual palpite:");

        let mut resposta = String::new();
        io::stdin().read_line(&mut resposta)
            .expect("Falha ao ler a linha");
        let resposta1 =  convert_to_int(&resposta);
        if resposta1 == 0{
            break;
        }

        if resposta1 > numero{
            println!("valor alto");
        }else if resposta1 < numero{
            println!("valor baixo"); 
        }else{
            println!("Acertou");
            break;
        }


        
        

        println!("Você palpitou: {}", resposta);

        
        }
    }






/*
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Digite seu palpite:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Falha ao ler a linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você palpitou: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("baixo!"),
            Ordering::Greater => println!("alto!"),
            Ordering::Equal => {
                println!("Você venceu!");
                break;
            }
        }
    }
}

*/