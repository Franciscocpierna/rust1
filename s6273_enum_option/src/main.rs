trait OptionExt<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T;

    fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U;
}

impl<T> OptionExt<T> for Option<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }

    fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Some(x) => Some(f(x)),
            None => None,
        }
    }
}

fn main() {
    let closure1 = || 2;

    let optional_number: Option<i32> = Some(42);

    // Exemplo de uso de unwrap_or_else()
    let result = optional_number.unwrap_or_else(closure1);
    println!("Valor resultante: {}", result);

    // Exemplo de uso de map()
    let mapped_result = optional_number.map(|x| x * 2);
    println!("Valor mapeado: {:?}", mapped_result);
}