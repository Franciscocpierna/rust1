use std::ops::Div;

#[derive(Debug, PartialEq)]
pub enum DivisionError{
    DivisionByZero,
    InvalidOperation,
    OutOfRange,
}
pub fn divide<T>(dividend: T, divisor: T) -> Result<T, DivisionError> 
   where 
    T: Div<Output = T> + PartialEq + From<u8> + Copy + PartialOrd,
{
 if divisor == T::from(0u8){
    return Err(DivisionError::DivisionByZero);
 }
 let result = dividend / divisor;
 if result == T::from(0u8){
    return Err(DivisionError::InvalidOperation);
 }

 if result > T::from(10u8){
   return Err(DivisionError::OutOfRange);
}
 Ok(result)
}

#[test]
fn test_divide(){
   assert_eq!(divide::<i32>(10, 2), Ok(5));
   assert_eq!(divide::<i32>(10, 0), Err(DivisionError::DivisionByZero));
   assert_eq!(divide(1, 3), Err(DivisionError::InvalidOperation));
   assert_eq!(divide(1000, 4), Err(DivisionError::OutOfRange));
}