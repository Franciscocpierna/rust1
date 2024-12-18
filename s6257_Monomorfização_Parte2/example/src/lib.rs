
pub trait Ola{
    fn ola(&self);
}

impl Ola for &str{
    fn ola (&self){
        println!("ola {} ", self)
    }
}

pub fn foo(){
    "J".ola();
}

pub fn bar(h: impl Ola){
     h.ola();
}

pub fn bar_str(h: &str){
    h.ola();
}

//Monomorfização cria duas copias de strlen