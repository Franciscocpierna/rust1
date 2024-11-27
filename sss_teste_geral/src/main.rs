

fn is_primo(n: u32) -> bool {
    let mut cont=0;
    if n <= 1 {
        return false;
    }
    for i in 2..(n / 2 + 1) {
        if n==31 {
            cont = cont + 1;
            println!("o numero: {} cont {} ", n, cont);
        }

        if n % i == 0 {
            return false;
        }
    }
    true
}


fn main() {
    let numeros: Vec<u32> = (2..50).collect();
    println!("Numeros {:?} ", numeros); 
    let primos_sequencial: Vec<u32> = numeros.iter().cloned().filter(|&x| is_primo(x)).collect();
    println!("Numeros {:?} ", primos_sequencial); 
}
