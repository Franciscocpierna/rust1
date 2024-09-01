





fn count(num: i32){
        
    for num in 1..11{
        println!("O numero é {}",num);
     }
}


fn count_down(num: &mut i32){
    while *num !=0
    {
        *num = *num -1;               
        println!("O numero é {}",num);
     }
}


fn main() {
   let mut num=10; 
   count(num);
   num=11;
   count_down(&mut num);




}

