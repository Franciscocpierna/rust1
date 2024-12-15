// Definição da estrutura Point
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// Implementação de métodos para Point
impl<X1, Y1> Point<X1, Y1> {
    // Método mixup
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}



#[test]
fn test_mixup() {
    // Criar pontos para teste
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    // Executar o método mixup
    let p3: Point<i32, char> = p1.mixup(p2);

    // Verificar se os valores foram misturados corretamente
    assert_eq!(p3.x, 5);  // x de p1
    assert_eq!(p3.y, 'c');  // y de p2
}
