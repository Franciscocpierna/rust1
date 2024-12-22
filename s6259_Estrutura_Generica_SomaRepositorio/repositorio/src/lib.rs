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

impl<T: std::ops::Add<Output = T> + Copy + Default> Repositorio<T> {
    // Método genérico para calcular a soma de todos os elementos do repositório.
    fn soma(&self) -> T {
        self.dados.iter().copied().fold(T::default(), |acc, x| acc + x)
    }
}

#[test]
fn test_soma_int() {
    let mut repo_int = Repositorio::novo(); // Criando um repositório para inteiros.
    repo_int.adicionar(1);
    repo_int.adicionar(2);
    repo_int.adicionar(3);
    assert_eq!(repo_int.soma(), 6);
}

#[test]
fn test_soma_float() {
    let mut repo_int = Repositorio::novo(); // Criando um repositório para inteiros.
    repo_int.adicionar(1.5);
    repo_int.adicionar(2.5);
    repo_int.adicionar(3.5);
    assert_eq!(repo_int.soma(), 7.5);
}