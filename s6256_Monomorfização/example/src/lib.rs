

pub fn strlen(s: impl AsRef<str>) -> usize{
    s.as_ref().len()
}

//primeira copia da função strlen()
pub fn strlen_refstr(s: &str) -> usize{
    s.len()
}

//segunda copia da função strlen()
pub fn strlen_string(s: String) -> usize{
    s.len()
}

pub fn strlen2<S>(s: S) -> usize where S: AsRef<str>{
    s.as_ref().len()
}

pub fn foo(){
    strlen("hello word"); // &'static str
    strlen(String::from("Bem-Vindo")) // String AsRef<str>
}

//Monomorfização cria duas copias de strlen