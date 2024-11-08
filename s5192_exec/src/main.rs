fn reorganizar_numero(num: i32) -> i32 {
    let mut digitos: Vec<char> = num.abs().to_string().chars().collect();

    if num >= 0 {
        digitos.sort();
        if let Some(pos) = digitos.iter().position(|&x| x != '0') {
            digitos.swap(0, pos);
        }
    } else {
        digitos.sort_by(|a, b| b.cmp(a));
    }

    let numero_reorganizado: String = digitos.iter().collect();
    let resultado = numero_reorganizado.parse::<i32>().unwrap();

    if num < 0 {
        -resultado
    } else {
        resultado
    }
}

fn main() {
    let num1 = 310;
    let num2 = -7605;
    
    println!("Entrada: {}, Saída: {}", num1, reorganizar_numero(num1)); // Saída: 103
    println!("Entrada: {}, Saída: {}", num2, reorganizar_numero(num2)); // Saída: -7650
}


/*
Você recebe um número inteiro num. Reorganize os dígitos de num de forma que seu valor seja minimizado e não contenha zeros à esquerda.

Retorne o número reorganizado com o valor mínimo.

Observe que o sinal do número não muda após a reorganização dos dígitos.

Exemplo 1:
Entrada:

num = 310
Saída:

103
Explicação: As possíveis arrumações para os dígitos de 310 são 013, 031, 103, 130, 301, 310. A arrumação com o menor valor que não contém zeros à esquerda é 103.

Exemplo 2:
Entrada:

num = -7605
Saída:

-7650
Explicação: Algumas possíveis arrumações para os dígitos de -7605 são -7650, -6705, -5076, -0567. A arrumação com o menor valor que não contém zeros à esquerda é -7650.
*/
