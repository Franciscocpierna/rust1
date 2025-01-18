#[derive(Debug)]
struct CommandDef {
    path: String,
    attrs: Vec<String>,
}

fn main() {
    // Definição de exemplo de command_defs
    let command_defs = vec![
        CommandDef {
            path: String::from("path1"),
            attrs: vec![String::from("attr1"), String::from("attr2")],
        },
        CommandDef {
            path: String::from("path2"),
            attrs: vec![String::from("attr3"), String::from("attr4")],
        },
    ];

    // Extração de paths e attrs utilizando map e unzip
    let (paths, attrs): (Vec<String>, Vec<Vec<String>>) = command_defs
        .into_iter()
        .map(|def| (def.path, def.attrs))
        .unzip();

    // Impressão dos resultados
    println!("Paths: {:?}", paths);
    println!("Attrs: {:?}", attrs);
}
