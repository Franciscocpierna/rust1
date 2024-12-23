pub trait Animal {
    fn fazer_barulho(&self);
}

pub struct Cachorro;
pub struct Gato;

impl Animal for Cachorro {
    fn fazer_barulho(&self) {
        println!("O cachorro faz au au!");
    }
}

impl Animal for Gato {
    fn fazer_barulho(&self) {
        println!("O gato faz miau!");
    }
}



#[test]
fn test_cachorro_faz_barulho() {
    let cachorro: Box<dyn Animal> = Box::new(Cachorro);
    assert_eq!(cachorro.fazer_barulho(), ());
}

#[test]
fn test_gato_faz_barulho() {
    let gato: Box<dyn Animal> = Box::new(Gato);
    assert_eq!(gato.fazer_barulho(), ());
}
