
fn converto_to_pig_latin(palavra:&String)->String{
    let vogais = ["a","e","i","o","u"];
    let (primeira_letra, resto_palavra)=palavra.split_at(1);
    let primeira_letra_vogal=vogais.contains(&primeira_letra);
    if primeira_letra_vogal{
        return format!("{}-hay", palavra);
    }
    return format!("{}-{}ay", resto_palavra,primeira_letra)
}



fn main() {
    let palavra= String::from("twitter");
    println!("{} em pig-latin eh {}", palavra, converto_to_pig_latin(&palavra));

}
