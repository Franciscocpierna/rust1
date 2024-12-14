fn maior<T> (a: T, b: T) -> T where T: PartialOrd {
  

    if a > b {
        a
    }else{
        b
    }
  
}


fn main() {
    let x = 5;
    let y = 10;
    println!("o maior numero é :{}",maior(x,y));
    
    let a = "hello";
    let b = "henrique";

    println!("A maior string eh : {}",maior (a,b));

    let x_float = 5.5;
    let y_float = 10.5;
    println!("o maior numero é :{}",maior(x_float,y_float));

    let a_char = 'a';
    let b_char = 'z';

    println!("A maior string eh : {}",maior (a_char,b_char));

    let x_vec = vec![1, 2, 3];
    let y_vec = vec![4, 5, 6];
    println!("o maior numero é :{:?}",maior(x_vec,y_vec));
    
    let x_tupla = (5, 'a');
    let y_tupla = (10, 'b');
    println!("o maior numero é :{:?}",maior(x_tupla,y_tupla));
}
