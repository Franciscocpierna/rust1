
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    hash: String,
    prev_hash: String,
}

impl Block {

    fn new(index: u64, timestamp: u64, data: String, hash: String, prev_hash: String) -> Block {

        Block {
            index,
            timestamp,
            data,
            hash,
            prev_hash,
        }
    }
    // Método que retorna o tamanho do dado (data) do bloco
    fn data_size(&self) -> usize {
        self.data.len()
    }

    // Método que retorna o tempo de criação do bloco em segundos
    fn creation_time(&self) -> u64 {
        self.timestamp / 1000
    }
}


fn main() {

    let my_block = Block::new(0, 1605991464000, "dados do bloco".to_string(),  "hash".to_string()  , "hash anterior".to_string());
    let size = my_block.data_size();
    let time = my_block.creation_time();
    println!("O tamanho do dado do bloco é: {} e foi criado em: {} segundos", size, time);
}

/*
Este código mostra como implementar um struct Block que representa um bloco de uma blockchain. O struct é definido na linha 4 e contém 5 campos: index, timestamp, data, hash, prev_hash.

Na linha 7, é implementado um método chamado new, que é usado para criar uma nova instância do struct Block com os dados especificados. Este método tem 4 parâmetros: index, timestamp, data, hash, prev_hash e retorna uma instância do struct Block com esses valores atribuídos aos campos correspondentes.

Na linha 14 e 17, são implementados os métodos data_size e creation_time. O método data_size retorna o tamanho dos dados (data) armazenados no bloco usando o método len() da string. O método creation_time retorna o tempo de criação do bloco em segundos dividindo o timestamp armazenado no struct Block por 1000.



Na função main é criada uma instância do struct Block usando o método new que foi criado, depois é chamado os métodos data_size e creation_time e armazena os valores retornados nas variáveis size e time. Por fim, esses valores são impressos na tela usando a macro println! para mostrar o tamanho dos dados e o tempo de criação do bloco.

Esse código é apenas uma base para criar uma blockchain, o cálculo do hash, e a validação do mesmo, não estão implementado, mas pode ser adicionado a esse struct de acordo com a necessidade. É importante notar que essa implementação é apenas ilustrativa e pode não ser segura o suficiente para uso em aplicações de produção, é necessário aprofundar-se na segurança e no funcionamento de uma blockchain antes de usá-lo em um projeto real.

*/