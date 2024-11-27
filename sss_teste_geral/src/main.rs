

fn is_primo(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..(n / 2 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}


fn main() {
    let primos_sequencial: Vec<u32> = numeros.iter().cloned().filter(|&x| is_primo(x)).collect();
}
