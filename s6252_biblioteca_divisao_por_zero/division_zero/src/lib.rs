use std::ops::Div;

#[derive(Debug, PartialEq)]
pub enum DivisionError{
    DivisionByZero,
}
pub fn divide<T>(dividend: T, divisor: T) -> Result<T, DivisionError> 
   where 
    T: Div<Output = T> + PartialEq + From<u8> + Copy,
{
 if divisor == T::from(0u8){
    return Err(DivisionError::DivisionByZero);
 }
 Ok(dividend / divisor) 
}

#[test]
fn test_divide(){
   assert_eq!(divide::<i32>(10, 2), Ok(5));
   assert_eq!(divide::<i32>(10, 0), Err(DivisionError::DivisionByZero));
}