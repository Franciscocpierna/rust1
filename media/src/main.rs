
fn media(numeros: &[f64]) -> f64{
    let mut mediar=0.0;
    let mut contador = 0;
    let mut tamanhovetor= numeros.len();
      

     
    while contador != tamanhovetor{
         mediar =mediar + numeros[contador];
         contador= contador + 1;
    }

    return mediar/contador as f64

}




fn main() {
    let numeros = [1.0,5.0,2.0,8.0,9.0];
    let recebe= media(&numeros);
    
    println!("A media é: {}", recebe);
}

/*
solução do professor

fn calcula_media(notas: &[f32]) -> f32 {
    let tamanho = notas.len();
    let mut soma: f32 = 0.0;
    for nota in notas {
        soma += *nota;
    }
    soma / tamanho as f32
}

fn main() {
    let notas = [7.5, 8.0, 9.0, 6.5];
    let media = calcula_media(&notas);
    println!("A média das notas é: {}", media);
}
*/
