/*Em Rust, a palavra-chave static é usada para criar variáveis de nível de módulo que são inicializadas quando o programa é iniciado e mantidas em memória durante toda a execução do programa. Essas variáveis são compartilhadas entre todas as instâncias de uma struct, enum ou trait e podem ser acessadas sem a necessidade de uma referência.

Aqui está um exemplo de como criar uma variável estática em Rust:

Copy code
static PI: f64 = 3.14159265;
Aqui, PI é uma variável estática de f64, que é inicializada com o valor 3.14159265.

As variáveis estáticas são úteis quando você deseja armazenar dados compartilhados entre as instâncias de uma struct ou quando deseja usar uma constante ao longo do seu código. Elas também podem ser usadas como contadores globais.

Além disso, as variáveis estáticas podem ser acessadas sem a necessidade de instância, usando apenas o nome da variavel precedido pelo nome do modulo:
mod my_module {
    static PI: f64 = 3.14159265;
}
//...
let result = my_module::PI * 2.0;
Além disso, existe também o conceito de static methods, que são metodos associados a uma struct ou enum e não precisam de uma instância para serem chamadas. Eles são declarados usando a sintaxe de ::

struct MyStruct;
impl MyStruct {
    fn new() -> MyStruct {
        MyStruct
    }
    fn do_something() {
       //...
    }
    fn do_something_with_self(self) {
       //...
    }
}
MyStruct::new();
MyStruct::do_something();
Aqui, o método new é um método construtor e o do_something é um método estático. O último exemplo é um erro pois o método do_something_with_self precisa de uma instância para ser chamado.

É importante notar que as variáveis estáticas precisam ser inicializadas com um valor constante e não podem ser mutáveis. Se você precisar modificar o valor de uma variável estática, é necessário usar um Mutex para fazer isso de forma segura.

Em resumo, as variáveis estáticas são úteis quando você deseja armazenar dados compartilhados entre as instâncias de uma struct ou quando deseja usar uma constante ao longo do seu código. Elas são inicializadas quando o programa é iniciado e mantidas em memória durante toda a execução do programa. Também é possível criar métodos estáticos, que são associados à uma struct ou enum e não precisam de uma instância para serem chamados.


*/

static mut STATIC_VARIAVEL: i32 =15;
fn main() {
    unsafe{
        println!("VALOR STATIC_VARIAVEL É {} ",STATIC_VARIAVEL);


     }
    // println!("VALOR STATIC_VARIAVEL É {}",STATIC_VARIAVEL); //var static tem que ser usada dentro do bloco
}
