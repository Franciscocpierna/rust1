fn max_coins(piles: Vec<i32>) -> i32 {
    // Ordenar as pilhas em ordem decrescente
    let mut sorted_piles = piles.clone();
    sorted_piles.sort_by(|a, b| b.cmp(a));
    println!("resultado sort {:?} ", sorted_piles);
    
    let mut max_coins = 0;
    let n = piles.len() / 3;
   // println!("valor de n {}", n);
    for i in 0..n {
        // Sempre pegar a segunda pilha mais alta do triplo
        max_coins += sorted_piles[2 * i + 1];
        println!("valor de max_coins {}", max_coins);
    }
    
    max_coins
}
   
    
fn main() {
    let piles1 = vec![2, 4, 1, 2, 7, 8];
    let piles2 = vec![2, 4, 5];
    let piles3 = vec![9, 8, 7, 6, 5, 1, 2, 3, 4];

    println!("Resultado para piles1: {}", max_coins(piles1)); // Deve imprimir 9
    println!("Resultado para piles2: {}", max_coins(piles2)); // Deve imprimir 4
    println!("Resultado para piles3: {}", max_coins(piles3)); // Deve imprimir 18
}