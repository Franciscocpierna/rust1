struct Solucao;

impl Solucao {
    pub fn dinheiro_minimo(transacoes: Vec<Vec<i32>>) -> i64 {
        let mut gastos_liquidos = 0; // gastos líquidos
        for t in &transacoes {
            gastos_liquidos += i32::max(0, t[0] - t[1]) as i64; // calcula os gastos líquidos
        }
        
        let mut dinheiro_minimo_necessario = gastos_liquidos;
        
        // Realiza a transação atual no final
        for transacao in transacoes {
            let custo = transacao[0];
            let cashback = transacao[1];
            
            if custo <= cashback {
                dinheiro_minimo_necessario = dinheiro_minimo_necessario.max(gastos_liquidos + custo as i64);
            } else {
                dinheiro_minimo_necessario = dinheiro_minimo_necessario.max(gastos_liquidos + cashback as i64);
            }
        }
        
        dinheiro_minimo_necessario
    }
}

fn main() {
    let transacoes1 = vec![
        vec![2, 1],
        vec![5, 0],
        vec![4, 2]
    ];
    
    let resultado1 = Solucao::dinheiro_minimo(transacoes1);
    println!("Dinheiro mínimo necessário: {}", resultado1); // Saída esperada: 10
    
    let transacoes2 = vec![
        vec![3, 0],
        vec![0, 3]
    ];
    
    let resultado2 = Solucao::dinheiro_minimo(transacoes2);
    println!("Dinheiro mínimo necessário: {}", resultado2); // Saída esperada: 3
}
