fn resolver_n_rainhas(n: i32) -> Vec<Vec<String>> {
    let mut resultado = Vec::new();
    let mut tabuleiro = vec![vec!['.'; n as usize]; n as usize];
    backtrack(&mut resultado, &mut tabuleiro, n as usize, 0);
    resultado
}

fn backtrack(resultado: &mut Vec<Vec<String>>, tabuleiro: &mut Vec<Vec<char>>, n: usize, linha: usize) {
    if linha == n {
        resultado.push(tabuleiro.iter().map(|linha| linha.iter().collect()).collect());
        return;
    }

    for coluna in 0..n {
        if eh_seguro(tabuleiro, linha, coluna, n) {
            tabuleiro[linha][coluna] = 'Q';
            backtrack(resultado, tabuleiro, n, linha + 1);
            tabuleiro[linha][coluna] = '.';
        }
    }
}

fn eh_seguro(tabuleiro: &Vec<Vec<char>>, linha: usize, coluna: usize, n: usize) -> bool {
    // Verifica se há rainhas na mesma coluna
    for i in 0..linha {
        if tabuleiro[i][coluna] == 'Q' {
            return false;
        }
    }

    // Verifica se há rainhas na diagonal esquerda
    let mut i = linha as i32 - 1;
    let mut j = coluna as i32 - 1;
    while i >= 0 && j >= 0 {
        if tabuleiro[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j -= 1;
    }

    // Verifica se há rainhas na diagonal direita
    let mut i = linha as i32 - 1;
    let mut j = coluna as i32 + 1;
    while i >= 0 && j < n as i32 {
        if tabuleiro[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j += 1;
    }

    true
}

fn main() {
    // Exemplo de uso:
    let n = 4;
    let solucoes = resolver_n_rainhas(n);
    for solucao in solucoes {
        for linha in solucao {
            println!("{}", linha);
        }
        println!();
    }
}
