use num_traits::{PrimInt, CheckedAdd,  Zero};

// Definição de um enum para representar os erros possíveis durante a divisão
#[derive(Debug, PartialEq)]
pub enum DivisionError {
    DivisionByZero,
    Overflow,

}

// Função para dividir dois números
pub fn divide<T>(dividend: T, divisor: T) -> Result<T, DivisionError>
where
    T: PrimInt + CheckedAdd  + PartialEq + Zero + Copy,
{
    // Verifica se o divisor é zero
    if divisor == T::zero() {
        return Err(DivisionError::DivisionByZero);
    }

    // Verifica se houve overflow ou underflow
    let check_overflow = dividend.checked_add(&divisor);

    if check_overflow.is_none() {
        return Err(DivisionError::Overflow);
    }


    // Realiza a divisão
    Ok(dividend / divisor)
}

#[test]
fn test_divide() {
    // Teste bem-sucedido de divisão
    assert_eq!(divide(10, 2), Ok(5));

    // Teste de divisão por zero
    assert_eq!(divide::<i32>(10, 0), Err(DivisionError::DivisionByZero));

    // Teste de divisão que resulta em overflow
    assert_eq!(divide::<i32>(i32::MAX, 1), Err(DivisionError::Overflow));

    
}