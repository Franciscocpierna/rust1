// Definindo a estrutura genérica Repositorio que pode armazenar elementos de qualquer tipo T.
struct Repositorio<T> {
    dados: Vec<T>,
}

impl<T> Repositorio<T> {
    // Método construtor para a estrutura Repositorio.
    fn novo() -> Self {
        Repositorio { dados: Vec::new() }
    }

    // Método para adicionar um novo item ao repositório.
    fn adicionar(&mut self, item: T) {
        self.dados.push(item);
    }
}

impl Repositorio<i32> {
    // Método específico para o tipo i32 que retorna a soma de todos os elementos do repositório.
    fn soma(&self) -> i32 {
        self.dados.iter().sum()
    }
}

impl Repositorio<f64> {
    // Método específico para o tipo f64 que retorna o produto de todos os elementos do repositório.
    fn produto(&self) -> f64 {
        self.dados.iter().product()
    }
}

#[test]
fn test_soma_int(){
    let mut repo_int = Repositorio::novo(); // Criando um repositório para inteiros.
    repo_int.adicionar(1);
    repo_int.adicionar(2);
    repo_int.adicionar(3);
    assert_eq!(repo_int.soma(), 6);

}

#[test]
fn test_float(){

    let mut repo_float = Repositorio::novo(); // Criando um repositório para floats.
    repo_float.adicionar(1.5);
    repo_float.adicionar(2.5);
    repo_float.adicionar(3.5);
    assert_eq!(repo_float.produto(), (1.5*2.5*3.5));

}