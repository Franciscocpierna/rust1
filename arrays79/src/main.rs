fn rotacionar(k: usize, numeros: &mut [i32]) {
    let mut i=0;
    let mut pega: Vec<i32>  = Vec::new();
    let mut valork = k; 
    let mut continua=0;
   
    for n in numeros.iter(){
        pega.push(*n);
        println!("{}", n);
    }
    numeros[k]=pega[numeros.len()-1];
    for m in pega.iter(){
        if valork < numeros.len()-1{
          numeros[valork+1]=pega[i];
          valork+=1;
        }else{
           if continua < k { 
                numeros[continua]=pega[i];
                continua+=1;
            
        }
        }
        
        i+=1

    }
     
    
    
}





fn main() {
    let mut numeros: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    let k=0;
    if k > (numeros.len()-1) || numeros.len()==0{
       println!("K tem que ser menor que vetor igual nem movimenta e não pode ser vazio")
    }else{
   
      println!("vetor origianal{:?}",numeros); 
       
      rotacionar(k,&mut numeros);
    
    
      println!("vetor retornado{:?}",numeros); 
    } 
     
     

    
  
}

/*
fn rotacionar_array(nums: &mut [i32; 7], k: usize) {
    let n = nums.len();

    if n == 0 {
        return; // Array vazio, não há nada para rotacionar
    }

    let k = k % n; // Lidar com casos em que k é maior que n

    nums.reverse(); // Inverter todo o array
    nums[0..k].reverse(); // Inverter os primeiros k elementos
    nums[k..].reverse(); // Inverter os elementos restantes
}

fn main() {
    let mut array = [1, 2, 3, 4, 5, 6, 7];
    let k = 1;

    println!("Array original: {:?}", array);

    rotacionar_array(&mut array, k);

    println!("Array rotacionado: {:?}", array);
}
*/