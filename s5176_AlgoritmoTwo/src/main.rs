
mod solution;


fn main() {
    /*let mut  v = vec![1, 2, 3, 4, 5];
    let alvo = 3;
    solution::Solution::two_sums(v, alvo);
    println!("Hello, world!"); */
    assert_eq!(solution::Solution::two_sums(vec![34,1,23,45,99], 100), vec![1,4]);
}
