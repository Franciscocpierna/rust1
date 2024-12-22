
pub trait draw{
    fn draw(&self);
}

pub struct Screen<T: Draw>{
    pub components: vec!<T>
} 

impl<T> Screen<T>
where:
     T: Draw,

{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();

        }
    }
}

fn main() {
    println!("Hello, world!");
}







/*
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),  
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("Ola")),
    //SpreadsheetCell::Float(10.34),
]
*/