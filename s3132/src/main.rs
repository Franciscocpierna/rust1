mod operations;
use operations::Operation;

fn main() {
    let operacao = Operation {addition:(4,5),subtraction:(6, 8),multiplication:(5, 4),division:(10, 2)};
    
    println!("Adicao {:?}", operacao.addition); 
    println!("Subtracao {:?}", operacao.subtraction);
    println!("Multiplicacao {:?}", operacao.multiplication);
    println!("Divisao {:?}", operacao.division); 
}








/*
pub enum Operation {
    Addition(i32, i32),
    Subtraction(i32, i32),
    Multiplication(i32, i32),
    Division(i32, i32),
}

pub fn calculate(op: Operation) -> Result<i32, &'static str> {
    match op {
        Operation::Addition(x, y) => Ok(x + y),
        Operation::Subtraction(x, y) => Ok(x - y),
        Operation::Multiplication(x, y) => Ok(x * y),
        Operation::Division(x, y) => {
            if y == 0 {
                Err("Cannot divide by zero.")
            } else {
                Ok(x / y)
            }
        }
    }
}

fn main() {
    let add = Operation::Addition(5, 3);
    let sub = Operation::Subtraction(5, 3);
    let mul = Operation::Multiplication(5, 3);
    let div = Operation::Division(5, 3);
    let div_by_zero = Operation::Division(5, 0);

    println!("5 + 3 = {}", calculate(add).unwrap());
    println!("5 - 3 = {}", calculate(sub).unwrap());
    println!("5 * 3 = {}", calculate(mul).unwrap());
    println!("5 / 3 = {}", calculate(div).unwrap());

    match calculate(div_by_zero) {
        Ok(result) => println!("5 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

*/