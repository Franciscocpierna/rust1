use std::collections::HashMap;

/// Struct para armazenar os dados e realizar cálculos estatísticos
pub struct EstatisticasDescritivas {
    numeros: Vec<i32>,
}

impl EstatisticasDescritivas {
    /// Construtor para inicializar a struct com um vetor de números
    pub fn new(numeros: Vec<i32>) -> Self {
        EstatisticasDescritivas { numeros }
    }

    /// Calcula a mediana do vetor de números
    pub fn mediana(&self) -> Result<f64, &'static str> {
        let mut numeros_sorted = self.numeros.clone();
        numeros_sorted.sort();

        if numeros_sorted.is_empty() {
            return Err("O vetor está vazio.");
        }

        let numero_meio = numeros_sorted.len() / 2;

        match numeros_sorted.len() % 2 {
            0 => {
                let media = EstatisticasDescritivas::media(&vec![
                    numeros_sorted[numero_meio] as f64,
                    numeros_sorted[numero_meio - 1] as f64,
                ]);
                Ok(media)
            }
            _ => Ok(numeros_sorted[numero_meio] as f64),
        }
    }

    /// Calcula a média do vetor de números em ponto flutuante
    pub fn media(numeros: &Vec<f64>) -> f64 {
        let soma: f64 = numeros.iter().sum();
        let quantidade = numeros.len() as f64;
        soma / quantidade
    }

    /// Calcula a moda (ou modas) do vetor de números
    pub fn moda(&self) -> Vec<i32> {
        let mut frequencia = HashMap::new();

        // Contagem das ocorrências de cada número
        for &numero in self.numeros.iter() {
            *frequencia.entry(numero).or_insert(0) += 1;
        }

        // Encontrando o valor com maior frequência
        let max_freq = frequencia.values().cloned().max().unwrap_or(0);

        // Filtrando os números que têm a maior frequência
        let moda: Vec<i32> = frequencia
            .into_iter()
            .filter(|&(_, freq)| freq == max_freq)
            .map(|(num, _)| num)
            .collect();

        moda
    }
}

/// Função que executa as estatísticas descritivas
pub fn executar_estatisticas_descritivas(numeros: Vec<i32>) {
    let estatisticas = EstatisticasDescritivas::new(numeros.clone());

    match estatisticas.mediana() {
        Ok(mediana) => println!("Mediana: {:.2}", mediana),
        Err(err) => println!("Erro ao calcular mediana: {}", err),
    };

    let numeros_f64: Vec<f64> = numeros.iter().map(|&x| x as f64).collect();
    let media_resultado = EstatisticasDescritivas::media(&numeros_f64);
    println!("Média: {:.2}", media_resultado);

    let moda_resultado = estatisticas.moda();
    println!("Moda: {:?}", moda_resultado);
}