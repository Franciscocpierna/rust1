fn dobro(num: i32) -> i32{
    return num*2;
}

fn maior(a: i32, b: i32) -> i32{
    if a >= b {
        return a;
    }else{
        return  b;
    }
}; 

fn alguma_fn(par_a: f32, par_b: i128) -> f32{
    println!("essa função devolve um valor flutuante"); 
   // 10.1 assim pode ser o retorno
   10 as f32 // assim para retornar esse inteiro como fosse f32
}
fn main() {
   /* let mut a=15;
    let mut b=40;
    while b != 0 {
        let temp = b;
        b = a%b;
        println!("b = {} ",b);
        a=temp;
    }
    println!("Maior divisor comum : {} ",a); */
    println!("o dobro de 5 é {}", dobro(5)); 
    println!("o dobro de 4 e 5 é {}", maior(4,5)); 
}
