use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Antes Closures {:?}", &list);
    
    thread::spawn(move || println!("From thread: {:?}",list))
    .join()
    .unwrap();
   //println!("Depois Closures {:?}", &list); // essa linha não imprime pq foi movido para thread
    
}
