use std::collections::HashMap;
 
fn main(){
    let mut hash_map = HashMap::new();
    hash_map.insert("Matematica", 90);
    hash_map.insert("Portugues", 72);
    hash_map.insert("Biologia", 58);
    hash_map.insert("Informatica", 96);
 
    println!("Quantas materias o aluno cursou? {}", hash_map.len());
 
    match hash_map.get("Informatica"){
        Some(k) => println!("O aluno cursou Informatica e tirou {}", k),
        None => println!("O aluno nao cursou Informatica")
    }
    hash_map.remove("Portugues");
    println!("o aluno estuda Portugues {} ", hash_map.contains_key("Portugues"));
    println!("o aluno estuda Matematica {} ", hash_map.contains_key("Matematica"));

    
}