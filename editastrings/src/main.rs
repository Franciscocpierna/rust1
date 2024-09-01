
fn esta_a_um_passo(str1: &str, str2: &str) -> bool {
    let len1 = str1.len();
    let len2 = str2.len();

    // Verifica se a diferença no comprimento é maior que 1
    if (len1 as i32 - len2 as i32).abs() > 1 {
        return false;
    }

    let mut edits = 0;
    let mut i = 0;
    let mut j = 0;

    // Percorre as strings para contar as edições
    while i < len1 && j < len2 {
        if str1.chars().nth(i) != str2.chars().nth(j) {
            edits += 1;

            // Se as strings têm comprimentos diferentes, avança apenas em uma delas
            if len1 > len2 {
                i += 1;
            } else if len1 < len2 {
                j += 1;
            } else {
                // Se as strings têm o mesmo comprimento, avança em ambas
                i += 1;
                j += 1;
            }
        } else {
            // Se os caracteres são iguais, avança em ambas as strings
            i += 1;
            j += 1;
        }

        // Se houver mais de uma edição, retorna falso
        if edits > 1 {
            return false;
        }
    }

    // Se as strings têm comprimentos diferentes e a diferença é de 1,
    // precisamos verificar se o último caractere é o mesmo nas duas strings
    if i < len1 || j < len2 {
        edits += 1;
    }

    // Retorna verdadeiro se houver no máximo uma edição
    edits <= 1
}

fn main() {
    let str1 = "pale";
    let str2 = "ple";
    println!("As strings estão a um passo de distância: {}", esta_a_um_passo(str1, str2));

    let str3 = "pales";
    let str4 = "pale";
    println!("As strings estão a um passo de distância: {}", esta_a_um_passo(str3, str4));

    let str5 = "pale";
    let str6 = "bale";
    println!("As strings estão a um passo de distância: {}", esta_a_um_passo(str5, str6));

    let str7 = "pale";
    let str8 = "bibo";
    println!("As strings estão a um passo de distância: {}", esta_a_um_passo(str7, str8));
}