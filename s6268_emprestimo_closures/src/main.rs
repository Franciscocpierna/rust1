fn main() {
    /*
    let exemple_closure = |x| x;  //recebe o parametro x e retorna ele
    let s = (exemple_closureString::from("hello"));
    println!("Saída para Exemplo 1: {}", s); */
    //let n = exemple_closure(5);esse não pode ser passado pq já foi criado como String
   
   // segundo exemplo funciona para todos
   /* let list = vec![1, 2, 3];
    println!("Antes de definir a closure: {:?}", &list); 
    let only_borrows = || println!("from closure {:?}", &list);
    println!("Antes de chamar a closure: {:?}", &list);
    only_borrows();
    println!("Depois de chamar a closure: {:?}", &list);    
    */
    // com list mutavel

    let mut list = vec![1, 2, 3];
    println!("Antes de definir a closure: {:?}", &list); //imutavel
    let mut only_borrows = || list.push(7); //mutavel
   // println!("Antes de chamar a closure: {:?}", &list); //emprestimo como imutavel não pode
    only_borrows();
    println!("Depois de chamar a closure: {:?}", &list);    

}
