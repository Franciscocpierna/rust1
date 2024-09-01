
fn maior_numero(numeros: &[i32])->i32{
      let mut maior = numeros[0];
      for num in numeros{
        if num > &maior{
            maior = *num;
        }
      }
      maior// ou return maior
}

fn main() {
   let mut numeros = [1,5,2,20,9,3];
   let maior = maior_numero(&mut numeros);
   println!("o maior número é : {} ", maior);
}

