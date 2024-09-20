















pub struct Operation{
   pub addition:(i32, i32),
   pub subtraction:(i32, i32),
   pub multiplication:(i32, i32),
   pub division:(i32, i32),
}



impl Operation{
    pub fn calculate(res: Operation) -> Result<i32, &'static str> {
        match res {
            Operation::addition(a, b) => Ok(a + b),
            Operation::subtraction(a, b) => Ok(a - b),
            Operation::multiplication(a, b) => Ok(a * b),
            Operation::division(a, b) => {
                if b == 0 {
                    Err("Cannot divide by zero.")
                } else {
                    Ok(a / b)
                }
            }
        }
}
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
*/