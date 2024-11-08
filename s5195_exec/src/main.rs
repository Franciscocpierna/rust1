use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

fn ler_arquivo(filename: &str) -> io::Result<Vec<i32>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let numeros: Vec<i32> = reader
        .lines()
        .filter_map(|line| line.ok()?.parse().ok())
        .collect();
    Ok(numeros)
}

fn bubble_sort(v: &mut Vec<i32>) {
    let n = v.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }
}

fn salvar_arquivo(numeros: &Vec<i32>, filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);

    for &numero in numeros {
        writeln!(writer, "{}", numero)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let arquivo = "numeros.txt";
    let mut numeros = ler_arquivo(arquivo)?;

    println!("Números antes da ordenação: {:?}", numeros);
    bubble_sort(&mut numeros);
    println!("Números depois da ordenação: {:?}", numeros);

    // Utilizando iteradores para exibir os números ordenados
    numeros.iter().for_each(|&num| print!("{} ", num));
    println!();

    salvar_arquivo(&numeros, "numeros_ordenados.txt")?;

    Ok(())
}