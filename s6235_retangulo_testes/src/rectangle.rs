// Implementação da estrutura Rectangle e seus métodos
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    // Método construtor
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rectangle { x, y, width, height }
    }

    // Método para calcular a área do retângulo
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    // Método para calcular o perímetro do retângulo
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    // Método para verificar se um ponto está dentro do retângulo
    pub fn contains_point(&self, point_x: f64, point_y: f64) -> bool {
        point_x >= self.x && point_x <= self.x + self.width &&
        point_y >= self.y && point_y <= self.y + self.height
    }

    // Método para imprimir os detalhes do retângulo
    pub fn print(&self) {
        println!("Rectangle: x={}, y={}, width={}, height={}", self.x, self.y, self.width, self.height);
    }
}