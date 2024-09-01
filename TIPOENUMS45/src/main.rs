/*enum Direction{
    Up,
    Down,
    Left,
    Right
}
 
 
fn main() {
 
    let player: Direction = Direction::Right;
    match player{
        Direction::Up => println!("O jogador foi para cima"),
        Direction::Down => println!("O jogador foi para baixo"),
        Direction::Right => println!("O jogador foi para direita"),
        Direction::Left => println!("O jogador foi para esquerda"),
    }
    
}*/
//cd\#[derive(Debug)]
/*enum Gender{
    Male, Female
}
 
 
fn main() {
 
    let player_male: Gender = Gender::Male;
    let player_female: Gender = Gender::Female;
    
    println!("{:?}", player_male);
    println!("{:?}", player_female);
    
}
*/
/*#[derive(Debug)]
enum CarType{
    Fiat,
    Ford,
    Renault,
}
 
fn nacionalidade_carro(car: CarType){
 
    match car{
        CarType::Fiat => println!("O carro eh italiano"),
        CarType::Ford => println!("O carro eh americano"),
        CarType::Renault => println!("O carro eh frances"),
    }
 
}
 
 
fn main() {
 
    nacionalidade_carro(CarType::Fiat);
    nacionalidade_carro(CarType::Ford);
    nacionalidade_carro(CarType::Renault);
    
}
*/
/*enum Pagamento{
    Dinheiro,
    CreditoCartao,
    DebitoCartao,
}
 
 
fn main() {
 
    let pessoa_pagamento = Pagamento::CreditoCartao;
    match pessoa_pagamento{
        Pagamento::Dinheiro => println!("A pessoa pagou em Dinheiro!"),
        Pagamento::CreditoCartao => println!("A pessoa pagou em Cartao de Credito!"),
        _ => {},
    }
    
}*/
enum Pagamento{
    Dinheiro(f32),
    CreditoCartao(bool, f32),
    DebitoCartao(bool, f32),
}
 
 
fn main() {
 
    let pessoa_pagamento = Pagamento::CreditoCartao(true,100f32);
    match pessoa_pagamento{
        Pagamento::Dinheiro(f) => println!("A pessoa pagou em Dinheiro {} reais!", f),
        Pagamento::CreditoCartao(true, x) => println!("A pessoa pagou em Cartao de Credito {}!", x),
        _ => {},
    }
    
}