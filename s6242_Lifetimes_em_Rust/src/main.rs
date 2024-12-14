// Definindo uma estrutura genérica `Par` que armazena uma referência a dois valores do mesmo tipo
struct Par<'a, T> {
    primeiro: &'a T,
    segundo: &'a T,
}

// Implementação da estrutura `Par`
impl<'a, T> Par<'a, T> {
    // Função associada que cria um novo `Par` com as referências especificadas
    fn novo(primeiro: &'a T, segundo: &'a T) -> Self {
        Par { primeiro, segundo }
    }

    // Função associada que retorna uma referência ao primeiro valor
    fn primeiro(&self) -> &'a T {
        self.primeiro
    }

    // Função associada que retorna uma referência ao segundo valor
    fn segundo(&self) -> &'a T {
        self.segundo
    }
}

fn main() {
    let x = 5;
    let y = 10;

    // Criando um `Par` de referências aos valores de `x` e `y`
    let par = Par::novo(&x, &y);

    // Imprimindo os valores contidos no `Par`
    println!("Primeiro valor: {}", par.primeiro());
    println!("Segundo valor: {}", par.segundo());
}