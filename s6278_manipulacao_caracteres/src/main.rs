#[derive(Debug, Clone)]
struct CharInfo {
    index: usize,
    character: char,
}

fn main() {
    let chars = vec!['A', 'B', 'C', 'D', 'E'];

    // Vetor para armazenar as informações de caracteres
    let mut char_infos: Vec<CharInfo> = Vec::new();

    // Index inicial
    let mut ix = 0;

    // Itera sobre os caracteres
    chars.iter()
        .map(|&ch| {
            ix += 1;
            CharInfo {
                index: ix,
                character: ch,
            }
        })
        .rev()
        .for_each(|info| {
            char_infos.push(info.clone()); // Armazena as informações em char_infos
            println!("PRINT: {:?}", info);
        });

    // Imprime o vetor de informações
    println!("Char Infos: {:?}", char_infos);
}
