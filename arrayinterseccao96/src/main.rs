


fn interseccao(nums: &mut Vec<i32>, nums1: &mut Vec<i32>)->Vec<i32>{
    let mut intersec: Vec<i32>  = Vec::new(); 
    let mut pega: Vec<i32> = Vec::new(); 
    
  for m in 0..nums1.len(){
    for n in 0..nums.len(){
       
        if nums1[m]== nums[n]{ 
         pega.push(nums[n]);
       
       }
     
    }
    
   }     
   pega.sort();
   intersec.push(pega[0]);
   for i in 0..pega.len(){
    if i+1 != pega.len(){ 
     if pega[i] != pega[i+1]{
        intersec.push(pega[i+1]);
     }
 }  
}  
return intersec;
  
  }





fn main() {
    let mut nums = vec![ 1, 2, 2, 1,];
    let mut nums1 = vec![2,2];
    
    
    // Chamar a função para mover os zeros para o final
    let result =interseccao(&mut nums, &mut nums1);

    // Imprimir o resultado
    println!("{:?}", result);
}
/*

use std::collections::HashSet;

fn intersecao(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // Converte os arrays para HashSet para busca eficiente
    let conjunto1: HashSet<_> = nums1.into_iter().collect();
    let conjunto2: HashSet<_> = nums2.into_iter().collect();

    // Utiliza o método intersection em HashSet para encontrar elementos comuns
    let resultado: Vec<_> = conjunto1.intersection(&conjunto2).cloned().collect();

    resultado
}

fn main() {
    // Exemplo de uso
    let nums1 = vec![1, 3, 4, 1, 5, 6];
    let nums2 = vec![2, 4, 5];

    // Chama a função para calcular a interseção
    let resultado = intersecao(nums1, nums2);

    // Imprime o resultado
    println!("Interseção: {:?}", resultado);
}


*/