fn minimum_lines(mut precos_acoes: Vec<Vec<i32>>) -> i32 {
    if precos_acoes.len() == 1 {
        return 0;
    }
    if precos_acoes.len() == 2 {
        return 1;
    }
    
    precos_acoes.sort_unstable();
    let mut res = 1;

    for i in precos_acoes.windows(3) {
        let esquerda = (i[1][1] - i[0][1]) as i64 * (i[2][0] - i[1][0]) as i64;
        let direita = (i[1][0] - i[0][0]) as i64 * (i[2][1] - i[1][1]) as i64;
        if esquerda != direita {
            res += 1;
        }
    }
    
    res
}

fn main() {
    let precos_acoes1 = vec![
        vec![1, 7],
        vec![2, 6],
        vec![3, 5],
        vec![4, 4],
        vec![5, 4],
        vec![6, 3],
        vec![7, 2],
        vec![8, 1],
    ];
    let resultado1 = minimum_lines(precos_acoes1);
    println!("Saída para o Exemplo 1: {}", resultado1); // Saída: 3

    let precos_acoes2 = vec![
        vec![3, 4],
        vec![1, 2],
        vec![7, 8],
        vec![2, 3],
    ];
    let resultado2 = minimum_lines(precos_acoes2);
    println!("Saída para o Exemplo 2: {}", resultado2); // Saída: 1
}

/*
Você recebe uma matriz 2D de inteiros stockPrices, onde stockPrices[i] = [dayi, pricei] indica que o preço da ação no dia dayi é pricei. Um gráfico de linha é criado a partir dessa matriz plotando os pontos em um plano XY, onde o eixo X representa o dia e o eixo Y representa o preço, conectando pontos adjacentes. Um exemplo é mostrado abaixo:


representando o preço. As seguintes 3 linhas podem ser desenhadas para representar o gráfico de linha:

Linha 1 (em vermelho) de (1,7) para (4,4) passando por (1,7), (2,6), (3,5) e (4,4).

Linha 2 (em azul) de (4,4) para (5,4).

Linha 3 (em verde) de (5,4) para (8,1) passando por (5,4), (6,3), (7,2) e (8,1). Pode-se mostrar que não é possível representar o gráfico de linha utilizando menos de 3 linhas.


Explicação: Como mostrado no diagrama acima, o gráfico de linha pode ser representado com uma única linha.
*/