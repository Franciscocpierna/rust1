use std::thread;

pub fn calculate_sum<'a, T>(numbers: &'a [T], num_threads: usize) -> T
where
    T: Default + Send + Sync + std::ops::Add<Output = T> + Copy + 'a + 'static,
{
    // Divide o vetor em partes iguais para cada thread
    let chunk_size = numbers.len() / num_threads;
    let mut threads = vec![];

    // Função para calcular a soma de uma parte do vetor
    let calculate_chunk_sum = |chunk: Vec<T>| -> T {
        chunk.iter().fold(T::default(), |acc, &x| acc + x)
    };

    // Cria uma nova thread para cada parte do vetor
    for chunk in numbers.chunks(chunk_size) {
        let chunk = chunk.to_vec(); // Cria uma nova alocação de memória para cada parte do vetor

        let handle = thread::spawn(move || calculate_chunk_sum(chunk));

        threads.push(handle);
    }

    // Aguarda todas as threads terminarem e coleta as somas parciais
    let mut partial_sums = vec![];
    for handle in threads {
        let partial_sum = handle.join().unwrap();
        partial_sums.push(partial_sum);
    }

    // Calcula a soma total combinando as somas parciais
    partial_sums.iter().fold(T::default(), |acc, &x| acc + x)
}


#[test]
fn test_calculate_sum() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let num_threads = 4;
    assert_eq!(calculate_sum(&numbers, num_threads), 55);
}
