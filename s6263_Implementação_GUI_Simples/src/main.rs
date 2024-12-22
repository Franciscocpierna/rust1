// Definição do trait Draw
trait Draw {
    fn draw(&self);
}

// Implementação do trait Draw para diferentes tipos
struct Button;
impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button");
    }
}

struct TextField;
impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing a text field");
    }
}

// Definição da struct Screen que contém uma lista de componentes genéricos que implementam o trait Draw
struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // Método para iterar sobre os componentes e chamar o método draw
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    // Criando uma instância de Screen com componentes de diferentes tipos
    let mut screen = Screen {
        components: vec![
            Box::new(Button),
            Box::new(TextField),
            Box::new(Button),
        ],
    };

    // Chamando o método run para desenhar os componentes na tela
    screen.run();
}
