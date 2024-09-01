fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut resultado = Vec::new();
    let mut subset_atual = Vec::new();
    
    fn backtrack(nums: &Vec<i32>, inicio: usize, subset_atual: &mut Vec<i32>, resultado: &mut Vec<Vec<i32>>) {
        resultado.push(subset_atual.clone());

        for i in inicio..nums.len() {
            subset_atual.push(nums[i]);
            println!("subset_atual {:?} i= {} ",subset_atual,i);
            backtrack(nums, i + 1, subset_atual, resultado);
            subset_atual.pop();
            println!("subset_atual é é {:?} i= {} ",subset_atual,i);
        }
    }
    
    backtrack(&nums, 0, &mut subset_atual, &mut resultado);
    resultado
}

fn main() {
    let nums1 = vec![1, 2, 3];
    let resultado1 = subsets(nums1);
    println!("{:?}", resultado1);

    let nums2 = vec![0];
    let resultado2 = subsets(nums2);
    println!("{:?}", resultado2);
}

