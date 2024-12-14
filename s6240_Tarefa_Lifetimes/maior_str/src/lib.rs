
fn maior_string<'a>(strings: &'a Vec<String>) -> Option<&'a String> {
    let mut maior: Option<&String> = None;
    for s in strings {
        match maior {
            Some(m) if m.len() < s.len() => {
                maior = Some(s);
            }
            None => {
                maior = Some(s);
            }
            _ => {}
        }
    }
    maior
}




#[test]
fn test_maior_string() {
    let vetor_vazio: Vec<String> = Vec::new();
    assert_eq!(maior_string(&vetor_vazio), None);

    let vetor = vec!["hello".to_string(), "henrique".to_string()];
    assert_eq!(maior_string(&vetor), Some(&"henrique".to_string()));
}
