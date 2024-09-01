fn encontrar_kesimo_maior(nums: &[i32], k: usize)->i32{
    let mut maior= nums[0]; 
    let mut contador=0;
    
    if maior > nums[1] && k==1{
        return maior;
    }else{
        if maior > nums[1]{
          contador+=1;
        }  
    }
    for m in 2..nums.len(){
        if maior < nums[m]{
         contador+=1;
         maior= nums[m];   
         if k == contador{
          return maior; 
         }  
        }
    
    }
   if k !=contador{ 
     println!("o {} k_esimo não encontrado ",k);
    }  
   return maior; 
}


fn main() {
    let nums = vec![3,2,1,5,6,4];
    let k=2;
    let resultado=encontrar_kesimo_maior(&nums,k);
    println!("o {} maior elemento é {} ",k , resultado);
    
}
/* solução do professor
fn encontrar_kesimo_maior(nums: &mut Vec<i32>, k: usize) -> i32 {
    // Ordena o array em ordem decrescente
    nums.sort_by(|a, b| b.cmp(a));

    // Retorna o k-ésimo maior elemento
    return nums[k - 1];
}

fn main() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 2;

    let resultado = encontrar_kesimo_maior(&mut nums.clone(), k);

    println!("O {}º maior elemento é: {}", k, resultado);
}

Seu envio
*/