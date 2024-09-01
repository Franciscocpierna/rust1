
fn mover_zeros(nums: &mut Vec<i32>){
    let mut movezero: Vec<i32>  = Vec::new(); 
    

    for n in nums.iter(){
       if *n !=0{ 
        movezero.push(*n);
       
       } 
    }
       
   for m in movezero.len()..nums.len(){
        
         movezero.push(0);
        
    } 
   for n in 0..movezero.len(){
         nums[n]=movezero[n];
         
    } 
 

    }





fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    
    // Chamar a função para mover os zeros para o final
    mover_zeros(&mut nums);

    // Imprimir o resultado
    println!("{:?}", nums);
}







/* exemplo professor
fn mover_zeros(nums: &mut Vec<i32>) {
    let mut indice_nao_zero = 0;

    // Iterar através do array
    for i in 0..nums.len() {
        // Se o elemento atual for não zero
        if nums[i] != 0 {
            // Trocar o elemento não zero com o elemento no índice indice_nao_zero
            nums.swap(i, indice_nao_zero);
            // Avançar o índice indice_nao_zero
            indice_nao_zero += 1;
        }
    }
}
fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    
    // Chamar a função para mover os zeros para o final
    mover_zeros(&mut nums);

    // Imprimir o resultado
    println!("{:?}", nums);
}


*/