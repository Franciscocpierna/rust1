use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let num_threads = 4;

    // Divide o vetor em partes iguais para cada thread
    let chunk_size = numbers.len() / num_threads;
    let mut threads = vec![];

    for chunk in numbers.chunks(chunk_size) {
        let chunk = chunk.to_vec();

        // Criando uma nova thread para calcular a soma parcial
        let handle = thread::spawn(move || {
            let sum = chunk.iter().fold(0, |acc, &x| acc + x);
            println!("Soma parcial: {}", sum);
            sum // Retorna a soma parcial
        });

        threads.push(handle);
    }

    // Aguarda todas as threads terminarem e coleta as somas parciais
    let mut partial_sums = vec![];
    for handle in threads {
        let partial_sum = handle.join().unwrap();
        partial_sums.push(partial_sum);
    }

    // Calcula a soma total combinando as somas parciais
    let total_sum: i32 = partial_sums.iter().sum();
    println!("Soma total: {}", total_sum);
}
