use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

fn main() -> Result<()> {
    // Abrindo o arquivo para leitura
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Coletando as linhas até encontrar uma linha vazia
    let rest_of_the_first_group: Result<Vec<_>> = reader
        .lines()
        .enumerate()
        .take_while(|(_, line)| match line {
            Ok(l) => !l.is_empty(),
            _ => true, // Em caso de erro, continue coletando
        })
        .map(|(index, line)| {
            line.map_err(|e| {
                // Convertendo o erro de leitura em um std::io::Error
                Error::new(ErrorKind::Other, format!("Error reading line {}: {}", index + 1, e))
            })
        })
        .collect();

    // Tratando o resultado da coleta
    match rest_of_the_first_group {
        Ok(lines) => {
            println!("Rest of the first group:");
            for line in &lines {
                println!("PRINT: {:?}", line);
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    Ok(())
}
