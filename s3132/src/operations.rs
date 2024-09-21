

pub struct Operation{
   pub primeiro: f64,
   pub segundo: f64,
}




  impl  Operation {
    pub fn addition(&self) -> f64{
       self.primeiro+self.segundo
    }

    pub fn subtraction(&self) -> f64{
        self.primeiro-self.segundo
    }
    pub fn multiplication(&self)-> f64{
        self.primeiro*self.segundo 
    }
    pub fn division(&self)-> f64{
        if self.segundo==0.0{
            println!("Não pode dividir por zero");
            return 0.0;
        }
        return self.primeiro/self.segundo;
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