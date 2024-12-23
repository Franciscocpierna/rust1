pub trait Media {
    fn reproduzir(&self);
}

pub struct Filme {
    titulo: String,
    duracao: u32, // em minutos
}

impl Filme {
    pub fn new(titulo: &str, duracao: u32) -> Self {
        Filme {
            titulo: titulo.to_string(),
            duracao,
        }
    }
}

impl Media for Filme {
    fn reproduzir(&self) {
        println!("Reproduzindo o filme '{}' ({} minutos)", self.titulo, self.duracao);
    }
}

pub struct Serie {
    titulo: String,
    num_temporadas: u8,
}

impl Serie {
    pub fn new(titulo: &str, num_temporadas: u8) -> Self {
        Serie {
            titulo: titulo.to_string(),
            num_temporadas,
        }
    }
}

impl Media for Serie {
    fn reproduzir(&self) {
        println!("Reproduzindo a série '{}' ({} temporadas)", self.titulo, self.num_temporadas);
    }
}



#[test]
fn test_reproduzir_filme() {
    let filme: Box<dyn Media> = Box::new(Filme::new("Matrix", 136));
    filme.reproduzir();
}

#[test]
fn test_reproduzir_serie() {
    let serie: Box<dyn Media> = Box::new(Serie::new("Breaking Bad", 5));
    serie.reproduzir();
}
