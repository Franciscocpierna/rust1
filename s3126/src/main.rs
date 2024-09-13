//extern create rand;
use rand::Rng;
use std::io;

fn main() {
    println!("Bem-vindo ao Jogo de Adivinhação de Palavra! Dica: Frutas!");

    let words = vec!["banana", "abacate", "uva", "laranja", "cacau", "caqui"];
    let secret_word = words[rand::thread_rng().gen_range(0, words.len())].to_string();
    let mut current_word = vec!['_'; secret_word.len()];

    loop {
        println!("Palavra corrente: {}", current_word.iter().collect::<String>());
        println!("Adivinhe uma letra:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().chars().next().unwrap();

        let mut found = false;
        for (i, c) in secret_word.chars().enumerate() {
            if c == guess {
                current_word[i] = c;
                found = true;
            }
        }

        if !found {
            println!("Letra não encontrada.");
        }

        if current_word.iter().collect::<String>() == secret_word {
            println!("Parabéns, você adivinhou a palavra {}!", secret_word);
            break;
        }
    }
}