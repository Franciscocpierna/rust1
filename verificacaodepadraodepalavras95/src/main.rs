use std::collections::{HashMap, HashSet};

fn word_pattern(pattern: &str, str_val: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let words: Vec<&str> = str_val.split_whitespace().collect();

    if pattern_chars.len() != words.len() {
        return false;
    }

    let mut char_to_word = HashMap::new();
    let mut word_to_char = HashMap::new();
    let mut used_words = HashSet::new();

    for (i, &ch) in pattern_chars.iter().enumerate() {
        match (char_to_word.get(&ch), word_to_char.get(&words[i])) {
            (Some(&word), Some(&character)) => {
                if word != words[i] || character != ch {
                    return false;
                }
            }
            (None, None) => {
                char_to_word.insert(ch, words[i]);
                word_to_char.insert(words[i], ch);
                used_words.insert(words[i]);
            }
            _ => return false,
        }
    }

    // Check if each character corresponds to a unique word and vice versa
    char_to_word.len() == used_words.len() && word_to_char.len() == used_words.len()
}

fn main() {
  
    let pattern1 = "abba";
    let str1 = "dog cat cat dog";
    println!("Segue o padrão: {}", word_pattern(pattern1, str1)); // Saída: true

    let pattern2 = "abba";
    let str2 = "dog cat cat fish";
    println!("Segue o padrão: {}", word_pattern(pattern2, str2)); // Saída: false

    let pattern3 = "aaaa";
    let str3 = "dog cat cat dog";
    println!("Segue o padrão: {}", word_pattern(pattern3, str3)); // Saída: false

    let pattern4 = "abba";
    let str4 = "dog dog dog dog";
    println!("Segue o padrão: {}", word_pattern(pattern4, str4)); // Saída: false

    
}
