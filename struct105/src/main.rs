

struct Produto {
     nome: String, 
     preco: f64,
     quantidade: u32,
}

impl Produto {
    fn new(nome: String ,preco:f64,quantidade: u32) -> Produto {
        Produto {nome,preco,quantidade }
    }

    fn get_nome(&self) -> String {
      return self.nome.clone()
    }

   fn set_nome(&mut self, nome1: &str) {
        self.nome= nome1.to_string();
    }
    fn get_quantidade(&self) -> u32 {
        self.quantidade
    }

   fn set_quantidade(&mut self, quantidade: u32) {
        self.quantidade= quantidade;
    }
    fn get_preco(&self) -> f64 {
        self.preco
    }

   fn set_preco(&mut self, preco: f64) {
        self.preco = preco;
    }
}


//mod produto; se for feito em 2 arquivos produto.rs
//use produto::Produto;
fn main() {
    let mut produto = Produto::new("Papel".to_string(),100.0,10);
    
    println!("Nome do Produto {}", produto.get_nome());
    println!("Preco do Produto: {}", produto.get_preco());
    println!("Quantidade do Produto: {}", produto.get_quantidade());

    produto.set_nome("novo papel");
    produto.set_preco(14.99);
    produto.set_quantidade(15);
    println!("novo nome  {}", produto.get_nome());
    println!("novo Preco: {}", produto.get_preco());
    println!("nova Quantidade: {}", produto.get_quantidade());


    
    
} 

/*

struct Produto {
    nome: String,
    preco: f32,
    quantidade: i32,
}

impl Produto {
    fn mudar_nome(&mut self, novo_nome: &str) {
        self.nome = novo_nome.to_string();
    }
    fn mudar_preco(&mut self, novo_preco: f32) {
        self.preco = novo_preco;
    }
    fn mudar_quantidade(&mut self, nova_quantidade: i32) {
        self.quantidade = nova_quantidade;
    }
    fn imprimir_dados(&self) {
        println!("Nome: {}", self.nome);
        println!("Preco: {}", self.preco);
        println!("Quantidade: {}", self.quantidade);
    }
}

fn main() {
    let mut meu_produto = Produto {
        nome: "Produto X".to_string(),
        preco: 10.99,
        quantidade: 20,
    };
    meu_produto.imprimir_dados();
    meu_produto.mudar_nome("Produto Y");
    meu_produto.mudar_preco(14.99);
    meu_produto.mudar_quantidade(15);
    meu_produto.imprimir_dados();
}

 struct Produto tem 3 campos: nome, preco e quantidade. A seguir, a cláusula impl é usada para adicionar métodos a struct. O método mudar_nome é usado para mudar o nome do produto, o método mudar_preco é usado para mudar o preco do produto, e o método mudar_quantidade é usado para mudar a quantidade do produto. O método imprimir_dados é usado para imprimir todos os dados do produto, imprimindo cada campo individualmente.

Na função main, é criada uma instância da struct Produto, e os métodos são chamados para testar se estão funcionando corretamente.

Observe que a cláusula '&mut self' é usada para que os métodos possam mudar o estado dos campos da struct e o '&self' é usado para que o metodo imprimir_dados não consiga mudar o estado dos campos da struct.

*/