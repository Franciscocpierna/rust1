struct Retangulo {
     largura: f64,
     altura: f64,
}
 
 
 trait Forma {
    fn area(&self) -> f64;
    fn perimetro(&self) -> f64;
    fn draw(&self);
}
 
impl Forma for Retangulo {
    fn area(&self) -> f64 {
        self.largura * self.altura
    }

    fn perimetro(&self) -> f64 {
        2.0 * (self.largura + self.altura)
    }

    fn draw(&self) {
        println!("Desenho de um retângulo de largura {} e altura {}", self.largura, self.altura);
    }
}

 
fn main(){
 

    
    let ret = Retangulo {
         largura: 10.0, 
         altura: 20.0 
        };

    

    println!("Área do retângulo: {}", ret.area());
    println!("Perímetro do retângulo: {}", ret.perimetro());
    ret.draw();
}
    


/*
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn draw(&self);
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * 3.14159 * self.radius
    }

    fn draw(&self) {
        println!("Desenhando um círculo de raio {}", self.radius);
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn draw(&self) {
        println!("Desenhando um retângulo de largura {} e altura {}", self.width, self.height);
    }
}

Crie o arquivo main.rs

mod shapes;

use shapes::{Shape, Circle, Rectangle};

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle { width: 10.0, height: 20.0 };

    println!("Área do círculo: {}", c.area());
    println!("Perímetro do círculo: {}", c.perimeter());
    c.draw();

    println!("Área do retângulo: {}", r.area());
    println!("Perímetro do retângulo: {}", r.perimeter());
    r.draw();
}



*/