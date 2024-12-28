struct MyOption<T>(Option<T>);

impl<T> MyOption<T>{
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self.0{
            Some(x) => x,
            None => f()
        } 
    }    
}



fn main() {
  //Definir a closure que será usada em unwrap_or_else
  let  fallback_value = || 42;
  //Testando com Some
  let some_value = MyOption(Some(10));
  let result_some = some_value.unwrap_or_else(fallback_value);
  println!("Result (Some) : {}", result_some);

  //Testando com None
  let none_value=MyOption(None);
  let result_none = none_value.unwrap_or_else(fallback_value);
  println!("Result (None) : {}", result_none)
}
