
fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut soma=0;
    let mut passodireita=0;
    let mut passoesquerda=0;
    let mut targetcont=1000;
    let mut direita=target;
    let mut esquerda = target;
    let mut distanciamenor=100000;
    let mut somar: Vec<i32> = Vec::new();
    for n in 0..nums.len(){
        
        
            if n+2 <= nums.len()-1 || n+1 < nums.len()-1{
               somar.push(nums[n]+nums[n+1]+nums[n+2]);
            }else{
                break;
            }

    

    }
    for n in 0..somar.len(){
        if direita == somar[n]{
            soma=somar[n];
            break
         }
        if distanciamenor == 100000 {
            soma=somar[n];
        } 
        while  targetcont==1000{
           direita+=1;
           passodireita+=1;
           passoesquerda+=1;
           esquerda-=1;
           if direita == somar[n]{
            targetcont=0; 
             if distanciamenor>passodireita{
                distanciamenor=passodireita;
                soma=somar[n];
             }
             
           }else if esquerda == somar[n] {
               targetcont=0;
            if distanciamenor>passoesquerda{
               distanciamenor=passoesquerda;
               soma=somar[n];
             }
             
           }
           
            
           }
           direita=target;
           passodireita=0;
           passoesquerda=0;
           esquerda=target; 
           targetcont=1000;
          
        }
        
        return soma;
   }
    





fn main() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;

    let result = three_sum_closest(nums, target);

    println!("A soma mais próxima do alvo é: {}", result);
}


/*

fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut closest_sum = i32::MAX;
    let mut min_diff = i32::MAX;

    for i in 0..nums.len() - 2 {
        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let current_sum = nums[i] + nums[left] + nums[right];
            let current_diff = (target - current_sum).abs();

            if current_diff < min_diff {
                closest_sum = current_sum;
                min_diff = current_diff;
            }

            if current_sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    closest_sum
}

fn main() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;

    let result = three_sum_closest(nums, target);

    println!("A soma mais próxima do alvo é: {}", result);
}

*/