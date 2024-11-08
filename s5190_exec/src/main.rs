use std::collections::HashMap;
use std::collections::HashSet;

fn encontrar_jogadores(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut derrotas = HashMap::new();
    let mut vencedores = HashSet::new();

    for match_result in matches {
        let vencedor = match_result[0];
        let perdedor = match_result[1];

        *derrotas.entry(perdedor).or_insert(0) += 1;
        vencedores.insert(vencedor);
    }

    let mut sem_derrotas = Vec::new();
    let mut uma_derrota = Vec::new();

    for &jogador in vencedores.iter() {
        if !derrotas.contains_key(&jogador) {
            sem_derrotas.push(jogador);
        }
    }

    for (&jogador, &num_derrotas) in derrotas.iter() {
        if num_derrotas == 1 {
            uma_derrota.push(jogador);
        }
    }

    sem_derrotas.sort();
    uma_derrota.sort();

    vec![sem_derrotas, uma_derrota]
}

fn main() {
    let matches1 = vec![
        vec![1, 3], vec![2, 3], vec![3, 6], vec![5, 6], vec![5, 7], vec![4, 5], vec![4, 8], vec![4, 9], vec![10, 4], vec![10, 9]
    ];
    let matches2 = vec![
        vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]
    ];

    let resultado1 = encontrar_jogadores(matches1);
    let resultado2 = encontrar_jogadores(matches2);

    println!("{:?}", resultado1); // Saída: [[1, 2, 10], [4, 5, 7, 8]]
    println!("{:?}", resultado2); // Saída: [[1, 2, 5, 6], []]
}

/*
Você recebe uma matriz de inteiros chamada matches, onde matches[i] = [vencedor_i, perdedor_i] indica que o jogador vencedor_i derrotou o jogador perdedor_i em uma partida.

Retorne uma lista answer de tamanho 2 onde:

answer[0] é uma lista de todos os jogadores que não perderam nenhuma partida.

answer[1] é uma lista de todos os jogadores que perderam exatamente uma partida.

Os valores nas duas listas devem ser retornados em ordem crescente.

Observações:

Considere apenas os jogadores que jogaram pelo menos uma partida.

Os casos de teste serão gerados de forma que nenhum jogo tenha o mesmo resultado.

Exemplo 1:
Entrada:

matches = [[1,3],[2,3],[3,6],[5,6],[5,7],[4,5],[4,8],[4,9],[10,4],[10,9]]
Saída:

[[1,2,10],[4,5,7,8]]
Explicação:

Os jogadores 1, 2 e 10 não perderam nenhuma partida.

Os jogadores 4, 5, 7 e 8 perderam exatamente uma partida.

Portanto, answer[0] = [1,2,10] e answer[1] = [4,5,7,8].

Exemplo 2:
Entrada:

matches = [[2,3],[1,3],[5,4],[6,4]]
Saída:

[[1,2,5,6],[]]
Explicação:

Os jogadores 1, 2, 5 e 6 não perderam nenhuma partida.

Nenhum jogador perdeu exatamente uma partida.

Portanto, answer[0] = [1,2,5,6] e answer[1] = [].


*/