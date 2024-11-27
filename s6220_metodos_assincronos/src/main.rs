use  tokio::time::{sleep, Duration};

async fn tarefa_assincrona(id: u32, duracao: u64){
   println!("Iniciando a tarefa {}.", id);
   sleep(Duration::from_secs(duracao)).await;
   println!("Tarefa {} concluida apos {} segundos", id, duracao);

}

#[tokio::main]
async fn main() {
    
    let tarefa1 = tokio::spawn(tarefa_assincrona(1, 3));
    let tarefa2 = tokio::spawn(tarefa_assincrona(2, 2));  
    
    let resultado1 = tarefa1.await.unwrap();
    let resultado2 = tarefa2.await.unwrap(); 

    println!("Resultado 1 {:?} ", resultado1);
    println!("Resultado 2 {:?} ", resultado2);
}
