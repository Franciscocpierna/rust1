struct Conta {
    nome: String,
    nrconta: u64,
    saldo: f64,
}

impl Conta {
    fn new(nome: String ,nrconta: u64,saldo:f64) -> Conta {
        Conta {nome,nrconta,saldo}
    }
    

    fn get_nome(&self) ->&String {
         &self.nome
      }
  
     fn set_nome(&mut self, nome1: &str){
          self.nome = nome1.to_string();
      }
      fn get_nrconta(&self) ->u64{
          return self.nrconta;
      }
      fn set_nrconta(&mut self,nrconta: u64 )  {
        self.nrconta=nrconta;
    }
      fn sacar(&mut self,valor:f64) {
        self.saldo-=valor;
      }
      fn depositar(&mut self, valor:f64){
         self.saldo=self.saldo+valor;
      }
      fn get_saldo(&self) ->f64{
          self.saldo
      }
  
     

    }





   



fn main() {
    let mut conta = Conta::new("Maria".to_string(),10,100.0); 
    println!("Nome  {}", conta.get_nome());
    println!("numero da conta: {}", conta.get_nrconta());
    println!("Saldo: {}", conta.get_saldo());

    conta.set_nome("novo nome");
    conta.depositar(14.0);
    conta.set_nrconta(15);
    println!("novo nome  {}", conta.get_nome());
    println!("novo nrconta: {}", conta.get_nrconta());
    conta.sacar(10.0);
    println!("novo Saldo: {}", conta.get_saldo());
    

    
}

/*professor
enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    CHF,
    AUD,
    CAD,
    BRL,
}

struct Account {
    balance: f64,
    currency: Currency,
}

impl Account {
    fn new(balance: f64, currency: Currency) -> Self {
        Self { balance, currency }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            return true;
        }
        return false;
    }

    fn check_balance(&self) -> f64 {
        self.balance
    }

    fn convert_to(&self, target_currency: Currency) -> f64 {
        // Utilize uma API de taxas de câmbio para converter o saldo para a moeda de destino
        let exchange_rate = 1.0; // exemplo
        self.balance * exchange_rate
    }
}


fn main() {
    let mut account = Account::new(1000.0, Currency::USD);
    println!("Saldo inicial: {}", account.check_balance());

    account.deposit(500.0);
    println!("Saldo após depósito: {}", account.check_balance());

    let withdrawal_success = account.withdraw(200.0);
    if withdrawal_success {
        println!("Saldo após saque: {}", account.check_balance());
    } else {
        println!("Saldo insuficiente para saque");
    }

    let converted_balance = account.convert_to(Currency::EUR);
    println!("Saldo convertido para EUR: {}", converted_balance);
}


*/