
/*Closures: funcoes anonimas . o uso esta em salvar em uma variavel
 ou passar como argumento de funcoes

 exemplo: Existe uma empresa que fornece camisetas exclusivas para uma lista de emails 
 de clientes. os clientes podem optar pela cor de camiseta que vai receber gratuitamente
 caso o cliente nao selecione a cor de camisa ira receber a cor que estiver em maior quantidade em estoque.

 */

#[derive(Debug, Copy, Clone)]
enum ShirtColor{
    Red,
    Blue,
}

struct Inventory{
    shirts: Vec<ShirtColor> 
}

impl Inventory{
    fn giveaway(&self, user_preference: Option<ShirtColor>)-> ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor{
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in  &self.shirts{
            match color {
               ShirtColor::Red => num_red += 1,
               ShirtColor::Blue => num_blue +=1,

            }
        }
        if num_red > num_blue{
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory{
       shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue], 
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("the user com preference {:?} gets {:?}", user_pref1, giveaway1); 

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("the user with preference {:?} gets {:?}", user_pref2, giveaway2); 

    let expensive_closure = |num: u32| -> u32{
        println!("Calculando...")
        thread::sleep(Duation::from_secs(2));
        num
    }
    //outros uso de closure
   /* 
    fn add_one_v1(x: u32) -> u32 {x + 1};
    let add_one_v2 = |x: u32| -> u32 {x + 1};
    let add_one_v3 = |x| {x+1};
    let add_one_v4 = |x| x+1;
   
    */
}
