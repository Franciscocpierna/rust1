
fn main() {

   let mut soma: i32 = 0;

    for y in 1..10{ 
        if y%2==0{
          soma += y;  
        }
    } 

    println!("A soma é : {} ", soma);
}

/*
solução do professor

fn main() {
    println!("Por favor, digite uma sequência de números reais:");

    let mut input = String::new();

    // Lê a sequência de números do usuário
    std::io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

    let mut numbers: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().expect("Por favor, insira números reais!"))
        .collect();

    // Calcula a soma dos números pares
    let mut sum: f64 = 0.0;
    for num in &numbers {
        if num % 2.0 == 0.0 {
            sum = sum + num;
        }
    }

    println!("A soma dos números pares é: {}", sum);


}
Método read_line: Este método lê uma linha da entrada padrão e armazena o conteúdo na variável `input`.



Método split_whitespace: Este método é usado para dividir a string `input` em um vetor de strings onde cada string é uma palavra individual.



Método parse::<f64>: Este método é usado para converter cada string para um número real (f64) e armazená-lo na variável `numbers`.



Método map: Este método é usado para aplicar a função `parse::<f64>` para cada elemento no vetor de strings.



Operador de módulo (%): Este operador é usado para calcular o resto da divisão de um número por outro. Se o resto da divisão é zero, significa que o número é par.

*/