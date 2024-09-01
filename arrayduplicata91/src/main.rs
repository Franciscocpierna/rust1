
fn contem_duplicatas(numeros: &mut Vec<i32>)->bool{
    numeros.sort();
   for i in 0..numeros.len(){
       if i+1 != numeros.len(){ 
        if numeros[i] == numeros[i+1]{
            return true;
        }
    }    
        
    }
    
    return false;

}

fn main() {
    let mut numeros = vec![1,2,3,4,5];
    let mut numeros1 = vec![1,2,3,4,1];
    println!("Array 1 contém duplicatas {} ", contem_duplicatas(&mut numeros));
    println!("Array 2 contém duplicatas {} ", contem_duplicatas(&mut numeros1));

}

/*
use std::collections::HashSet;

fn contains_duplicates(nums: &[i32]) -> bool {
    let mut seen = HashSet::new();

    for &num in nums {
        if !seen.insert(num) {
            //se o valor já exite no HashSet tem duplicata
            return true;
        }
    }

    // não foi encontrada duplicatas
    false
}

fn main() {
    let nums1 = vec![1, 2, 3, 4, 5];
    let nums2 = vec![1, 2, 3, 4, 1];

    println!("Array 1 contains duplicates: {}", contains_duplicates(&nums1));
    println!("Array 2 contains duplicates: {}", contains_duplicates(&nums2));
}


*/