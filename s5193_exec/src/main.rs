use std::collections::HashMap;

pub fn melhores_alunos(
    feedback_positivo: Vec<&str>,
    feedback_negativo: Vec<&str>,
    relatorio: Vec<&str>,
    id_aluno: Vec<i32>,
    k: i32,
) -> Vec<i32> {
    let mut conjunto_positivo = std::collections::HashSet::new();
    let mut conjunto_negativo = std::collections::HashSet::new();

    for palavra in feedback_positivo {
        conjunto_positivo.insert(palavra);
    }

    for palavra in feedback_negativo {
        conjunto_negativo.insert(palavra);
    }

    let mut pontuacoes = HashMap::new();

    for (i, rel) in relatorio.iter().enumerate() {
        let id = id_aluno[i];
        let palavras: Vec<&str> = rel.split_whitespace().collect();

        for palavra in palavras {
            if conjunto_positivo.contains(palavra) {
                *pontuacoes.entry(id).or_insert(0) += 3;
            } else if conjunto_negativo.contains(palavra) {
                *pontuacoes.entry(id).or_insert(0) -= 1;
            }
        }
    }

    let mut alunos: Vec<(i32, i32)> = pontuacoes.into_iter().collect();
    alunos.sort_by(|a, b| {
        if b.1 == a.1 {
            a.0.cmp(&b.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    alunos.into_iter().take(k as usize).map(|(id, _)| id).collect()
}

fn main() {
    let feedback_positivo = vec!["inteligente", "brilhante", "estudioso"];
    let feedback_negativo = vec!["não"];
    let relatorio = vec!["este aluno é estudioso", "o aluno é inteligente"];
    let id_aluno = vec![1, 2];
    let k = 2;
    let resultado = melhores_alunos(feedback_positivo, feedback_negativo, relatorio, id_aluno, k);
    println!("{:?}", resultado); // Saída: [1, 2]
}


/*
Você tem listas de palavras que representam feedback positivo e negativo. Cada aluno começa com 0 pontos, e os pontos são ajustados com base nos relatórios de feedback: palavras positivas aumentam os pontos em 3, enquanto palavras negativas diminuem os pontos em 1.

Para cada relatório, o ID do aluno correspondente é dado. Após calcular os pontos para todos os alunos, você deve retornar os IDs dos top K alunos com a maior pontuação. Se dois alunos tiverem a mesma pontuação, o aluno com o menor ID será classificado mais alto.



Exemplo 1:
Entrada:

feedback_positivo = ["inteligente", "brilhante", "estudioso"]
feedback_negativo = ["não"]
relatorio = ["este aluno é estudioso", "o aluno é inteligente"]
id_aluno = [1, 2]
k = 2
Saída:

código[1, 2]
Explicação: Ambos os alunos têm 1 feedback positivo e 3 pontos, mas como o aluno 1 tem um ID menor, ele se classifica mais alto.

Exemplo 2:
Entrada:

feedback_positivo = ["inteligente", "brilhante", "estudioso"]
feedback_negativo = ["não"]
relatorio = ["este aluno não é estudioso", "o aluno é inteligente"]
id_aluno = [1, 2]
k = 2
Saída:

código[2, 1]
Explicação:

O aluno com ID 1 tem 1 feedback positivo e 1 feedback negativo, então ele tem 3-1=2 pontos.

O aluno com ID 2 tem 1 feedback positivo, então ele tem 3 pontos. Como o aluno 2 tem mais pontos, [2, 1] é retornado.

*/
