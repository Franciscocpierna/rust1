use std::collections::HashMap;

fn can_construct(ransom_note: &str, magazine: &str) -> bool {
    let mut magazine_chars = HashMap::new();

    // Contagem de caracteres na revista
    for ch in magazine.chars() {
        *magazine_chars.entry(ch).or_insert(0) += 1;
    }

    // Verificação da construção da nota de resgate
    for ch in ransom_note.chars() {
        if let Some(count) = magazine_chars.get_mut(&ch) {
            if *count == 0 {
                return false; // Não há caracteres suficientes na revista
            }
            *count -= 1;
        } else {
            return false; // Caractere não encontrado na revista
        }
    }

    true
}

fn main() {
    // Exemplos de teste
    let ransom_note1 = "a";
    let magazine1 = "b";
    println!("{}", can_construct(ransom_note1, magazine1)); // Saída: false

    let ransom_note2 = "aa";
    let magazine2 = "ab";
    println!("{}", can_construct(ransom_note2, magazine2)); // Saída: false

    let ransom_note3 = "aa";
    let magazine3 = "aab";
    println!("{}", can_construct(ransom_note3, magazine3)); // Saída: true
}
