trait Account {
  fn deposit(&mut self, amount: f64);
  fn withdraw(&mut self, amount: f64) ;
  fn balance(&self) -> f64;
}

struct BankAccount {
  account_number: u32,
  holder_name: String,
  balance: f64,
}

impl BankAccount{
  fn new(account_number: u32, holder_name: &str, initial_balance: f64) -> Self{
    BankAccount { account_number, holder_name: holder_name.to_string(), balance: initial_balance }
  }
}

impl Account for BankAccount{
  fn deposit(&mut self, amount: f64) {
   self.balance += amount; 
}
  fn withdraw(&mut self, amount: f64) {
   self.balance -= amount;
}
  fn balance(&self) -> f64{
   self.balance 
}
}

fn main() {
  let mut account1 = BankAccount::new(17, "Steve Jobs",  150.0);
  let mut account2 = BankAccount::new(28, "John Smith",  360.0);

  account1.deposit(80.0);
  account2.withdraw(15.0);

  println!("balance of account1: {} \nbalance of account2: {}", account1.balance, account2.balance);
}
