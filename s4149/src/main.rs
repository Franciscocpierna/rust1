mod lib;
use lib::{EstatisticasDescritivas};
use lib::executar_estatisticas_descritivas;
//use s4149::executar_estatisticas_descritivas; acima ou essa

fn main() {
    let numeros = vec![1, 1, 1, 2, 3, 4, 4, 4, 5, 6, 0];
    executar_estatisticas_descritivas(numeros);
}