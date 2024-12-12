
pub struct Point{
    pub x: f64,
    pub y: f64
}
impl Point{
    // metodo construtor
    pub fn new(x: f64, y: f64) -> Self {
        Point{x, y}
    }

    // metodo calcular distância entre dois pontos
    // d=√((x_2-x_1)²+(y_2-y_1)²)
    pub fn distance(&self, other: &Point) -> f64{
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    //metodo para mover para proxima posição
    pub fn move_to(&mut self,x: f64, y: f64){
        self.x = x;
        self.y = y;

    }

    // metodo para imprimir as coordenadas do ponto
    pub fn print(&self){
        println!("({}, {})", self.x, self.y);
    }
}