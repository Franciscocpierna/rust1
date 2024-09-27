// trait chamado "BankAccount" 
//funções new(balance: f64, currency: Currency) -> Self" 
//que cria uma nova instância de conta bancária com o saldo e a moeda passados como parâmetro.

//deposit(&mut self, amount: f64)", que permite depositar dinheiro na conta.

//withdraw(&mut self, amount: f64) -> bool", que permite sacar dinheiro da conta, retornando 
//"true" se o saque foi realizado com sucesso e "false" se o saldo for insuficiente.

//check_balance(&self) -> f64", que retorna o saldo atual da conta.

//enum chamado "Currency", que contém as opções de moedas disponíveis para a conta (USD, EUR, GBP, etc).

//struct chamada "Account" que implementa o trait "BankAccount" 

//contém as seguintes características:

//Um campo "balance" para armazenar o saldo da conta.

//Um campo "currency" para armazenar a moeda da conta.

pub trait BankAccount{
    fn new(balance: f64,currency: Currency)-> Self;
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> bool;
    fn check_balance(&self) -> f64;    

}

pub enum Currency {
    USD,
    EUR,
    GBP,

}

pub struct Account {
    balance: f64,
    currency: Currency
}

impl BankAccount for Account {
    fn new(balance: f64,currency: Currency)-> Self{
        Account{
            balance,
            currency,
        }
          
    }

    fn deposit(&mut self, amount: f64){
        self.balance+=amount

    }

    fn withdraw(&mut self, amount: f64) -> bool{
          if self.balance < amount{
            false
          }else{
            self.balance-=amount; 
            true
          }
          

    }   
    fn check_balance(&self) -> f64{
        self.balance
    }
    
}

/*
pub trait BankAccount {
    fn new(balance: f64, currency: Currency) -> Self;
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> bool;
    fn check_balance(&self) -> f64;

}

pub enum Currency {
    USD,
    EUR,
    GBP,
    // outras moedas
}

pub struct Account {
    balance: f64,
    currency: Currency,
}

impl BankAccount for Account {
    fn new(balance: f64, currency: Currency) -> Self {
        Account { balance, currency }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        match self.balance > amount {
            true => {
                self.balance -= amount;
                true
            },
            false => false,
        }
    }

    fn check_balance(&self) -> f64 {
        self.balance
    }

}




*/