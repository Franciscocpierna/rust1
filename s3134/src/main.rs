

mod account;
use account::{Account,Currency};
use account::BankAccount;


fn main(){
 let mut conta = Account::new(1000.0,Currency::USD);
 let mut conta1=Account::new(100.00,Currency::USD);
 
 conta.deposit(500.00);

 if conta.withdraw(1200.00){
    println!("retirada com Sucesso");
    println!("Saldo da conta {:.2} ",conta.check_balance());
 }else{
    println!("falhou a retirada");
    println!("Saldo da conta {:.2}",conta.check_balance());
 }
println!("");

if conta.withdraw(1000.00){
    println!("retirada com Sucesso");
    println!("Saldo da conta {:.2}",conta.check_balance());
 }else{
    println!("falhou a retirada");
    println!("Saldo da conta {:.2}",conta.check_balance());
 }
 println!("");

 

conta.deposit(300.00);

 println!("");

 if conta1.withdraw(30.00){
    println!("retirada com Sucesso");
    println!("Saldo da conta1 {:.2}",conta1.check_balance());
 }else{
    println!("falhou a retirada");
    println!("Saldo da conta1 {:.2}",conta1.check_balance());
 }
 println!("");

conta1.deposit(2000.00);

if conta1.withdraw(1000.00){
    println!("retirada com Sucesso");
    println!("Saldo da conta1 {:.2}",conta1.check_balance());
 }else{
    println!("falhou a retirada");
    println!("Saldo da conta1 {:.2}",conta1.check_balance());
 }
 println!("");

 let recebesaldoconta= conta.check_balance();
 let recebesaldoconta1=conta1.check_balance();
 println!("Saldo da conta final {:.2}", recebesaldoconta);
 println!("Saldo da conta1 final {:.2}", recebesaldoconta1); 


}




/*mod account;

use account::{BankAccount, Currency, Account};

fn main() {
    let mut account = Account::new(1000.0, Currency::USD);

    account.deposit(500.0);
    println!("New balance: {:.2}", account.check_balance());

    let withdrawal_success = account.withdraw(1200.0);
    if withdrawal_success {
        println!("Withdrawal success, new balance: {:.2}", account.check_balance());
    } else {
        println!("Withdrawal failed, insufficient funds.");
    }


}
*/
