use  tokio::time::{sleep, Duration};
use  reqwest::Error;

async fn fazer_solicitacao(id: u32, url: &str) -> Result<(), Error>{
println!("Iniciando a solicitacao {} para{}", id, url);
// Simular um trabalho assincrono (poderia ser uma solicitação HTTP)
sleep(Duration::from_secs(2)).await;

// Exemplo de uma solicitação HTTP assincrona usando reqwest
let resposta = reqwest::get(url).await?; 
// Manipula a resposta conforme necessario (aqui imprime apenas o status)
println!("Resposta da solicitação {}: {:?}",id, resposta.status());

Ok(())

} 

#[tokio::main]
async fn main() {
    let solicitacao1 = tokio::spawn(fazer_solicitacao(1, "https://www.example.com"));
    let solicitacao2 = tokio::spawn(fazer_solicitacao(2, "https://www.rust-lang.org"));

    //aguardar as solicitações
    let resultado1 = solicitacao1.await.unwrap();
    let resultado2 = solicitacao2.await.unwrap();
    
    if resultado1.is_ok(){
      println!("solicitação 1 foi concluida com sucesso"); 
    }else{
       println!("erro na solicitação 1 {:?} ", resultado1); 
    };
    if resultado2.is_ok(){
      println!("solicitação 2 foi concluida com sucesso"); 
    }else{
        println!("erro na solicitação 2 {:?} ", resultado2); 
    };
}

