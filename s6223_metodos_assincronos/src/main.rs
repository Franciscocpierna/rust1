use  tokio::time::{sleep, Duration};

async fn tarefa_assincrona(id: u32, duracao: u64){
   println!("Iniciando a tarefa {}.", id);
   sleep(Duration::from_secs(duracao)).await;
   for _ in 0..10{
    println!(""); 
   };
   println!("Tarefa {} concluida apos {} segundos", id, duracao);

}

#[tokio::main]
async fn main() {
    
    let tarefa1 = tokio::spawn(tarefa_assincrona(1, 3));
    let tarefa2 = tokio::spawn(tarefa_assincrona(2, 2)); 
    let tarefa3 = tokio::spawn(tarefa_assincrona(3, 1));
    let tarefa4 = tokio::spawn(tarefa_assincrona(4, 4));  
    
    let resultado1 = tarefa1.await.unwrap();
    let resultado2 = tarefa2.await.unwrap(); 
    let resultado3 = tarefa3.await.unwrap();
    let resultado4 = tarefa4.await.unwrap(); 

    println!("Resultado 1 {:?} ", resultado1);
    println!("Resultado 2 {:?} ", resultado2);
    println!("Resultado 3 {:?} ", resultado3);
    println!("Resultado 4 {:?} ", resultado4);
}

/*
use tokio::time::{sleep, Duration};

async fn tarefa(id: u32, duracao: u64) {
    println!("Iniciando tarefa {}.", id);

    // Introduz um atraso assíncrono
    sleep(Duration::from_secs(duracao)).await;

    println!("Tarefa {} concluída após {} segundos.", id, duracao);
}

#[tokio::main]
async fn main() {
    // Inicia três tarefas assíncronas simultaneamente
    let tarefa1 = tokio::spawn(tarefa(1, 3));
    let tarefa2 = tokio::spawn(tarefa(2, 2));
    let tarefa3 = tokio::spawn(tarefa(3, 1));

    // Aguarda a conclusão de todas as tarefas
    let _ = tokio::try_join!(tarefa1, tarefa2, tarefa3)
        .map_err(|e| println!("Erro ao aguardar tarefas: {:?}", e))
        .unwrap();
}


*/
