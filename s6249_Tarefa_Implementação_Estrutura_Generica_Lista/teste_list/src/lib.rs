// Definição de uma estrutura genérica Lista
struct Lista<T> {
    elementos: Vec<T>,
}

impl<T> Lista<T> {
    // Função associada para criar uma nova Lista vazia
    fn nova() -> Self {
        Lista { elementos: Vec::new() }
    }

    // Função associada para adicionar um elemento à Lista
    fn adicionar(&mut self, elemento: T) {
        self.elementos.push(elemento);
    }

    // Função associada para remover um elemento da Lista
    fn remover(&mut self, indice: usize) -> Option<T> {
        if indice < self.elementos.len() {
            Some(self.elementos.remove(indice))
        } else {
            None
        }
    }

    // Função associada para buscar um elemento na Lista
    fn buscar(&self, elemento: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.elementos.iter().position(|e| e == elemento)
    }

    // Função associada para ordenar os elementos da Lista
    fn ordenar(&mut self)
    where
        T: Ord,
    {
        self.elementos.sort();
    }
}


#[test]
fn test_adicionar_e_buscar() {
    let mut lista = Lista::nova();
    lista.adicionar(5);
    lista.adicionar(10);
    lista.adicionar(15);
    assert_eq!(lista.buscar(&10), Some(1));
}

#[test]
fn test_remover() {
    let mut lista = Lista::nova();
    lista.adicionar(5);
    lista.adicionar(10);
    lista.adicionar(15);
    lista.remover(1);
    assert_eq!(lista.elementos, vec![5, 15]);
}

