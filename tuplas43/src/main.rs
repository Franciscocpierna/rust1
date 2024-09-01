fn main() {
    let tupla= (12,"valores",3.14,(1, 2, 3));
    let (a, b, c, d)=tupla;
    println!("{}", tupla.3.2);
    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
    println!("{:?}",d);
}
