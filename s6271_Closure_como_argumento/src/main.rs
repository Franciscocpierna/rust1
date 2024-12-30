struct MyResult<T, E>(Result<T, E>);

impl<T, E> MyResult<T, E> {
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self.0 {
            Ok(x) => x,
            Err(e) => f(e),
        }
    }
}

fn main() {
    // Define uma função que será usada como argumento para unwrap_or_else
    let fallback_value = |err| {
        println!("Erro: {}", err);
        42
    };

    // Testando com Ok
    let ok_value = MyResult(Ok(10));
    let result_ok = ok_value.unwrap_or_else(fallback_value);
    println!("Resultado (Ok): {}", result_ok); // Saída esperada: 10

    // Testando com Err
    let err_value = MyResult(Err("Erro interno"));
    let result_err = err_value.unwrap_or_else(fallback_value);
    println!("Resultado (Err): {}", result_err); // Saída esperada: 42
}