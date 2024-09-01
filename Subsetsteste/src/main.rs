
fn subsets(teste: &i32 ) -> i32 {
    let mut vezes=0;
    let mut  i =0;
    fn backtrack(teste: &i32,inicio: usize,vezes: &mut i32, i: &mut i32) {
        
        *i=*i+1;
        
        for j in inicio..2 {
            println!("entrou nr vezes de j {} ",j);
            backtrack(teste,j+1,vezes, i);
            *vezes=*vezes +1;
            println!("entrou nr vezes de vezes {} ",vezes);
        }
    }
    
    backtrack(&teste,0,&mut vezes, &mut i);
    vezes
}

fn main() {
    let  teste : i32= 0;
    let resultado2 = subsets(&teste);
    println!("{}", resultado2);
}

