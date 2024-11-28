use tokio::time::{sleep, Duration};

async fn tarefa(id: u32, duracao: u64) -> Result<(), &'static str> {
    println!("Iniciando tarefa {}.", id);

    // Introduz um atraso assíncrono
    sleep(Duration::from_secs(duracao)).await;

    if duracao % 2 == 0 {
        // Simula um erro para tarefas com duração par
        Err("Erro: Duração par não é permitida.")
    } else {
        println!("Tarefa {} concluída após {} segundos.", id, duracao);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let tarefa1 = tokio::spawn(tarefa(1, 3));
    let tarefa2 = tokio::spawn(tarefa(2, 2));
    let tarefa3 = tokio::spawn(tarefa(3, 1));

    // Aguarda a conclusão de todas as tarefas
    let resultado = tokio::try_join!(tarefa1, tarefa2, tarefa3)
        .map_err(|erro| println!("Erro ao aguardar tarefas: {:?}", erro))
        .unwrap();

    println!("Resultados: {:?}", resultado);
}
