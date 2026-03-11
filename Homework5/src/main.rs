#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount <= 0.0{
            panic!("You can't deposit negative money silly!!");
        }
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) {
        if self.balance <= 0.0{
            panic!("You don't have money to withdraw!!");
        }
        else if amount < 0.0{
            panic!("You can't withdraw negative money silly!!");
        }
        else if self.balance < amount{
            panic!("You don't have that much money to withdraw!");
        }
        self.balance -= amount;
    }

    pub fn balance(&self) -> f64 {
       self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let test = BankAccount::new(20.0);
         assert!((test.balance() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
         let mut test = BankAccount::new(50.0);
         test.deposit(50.0);
         assert!((test.balance()- 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut test = BankAccount::new(50.0);
         test.withdraw(50.0);
         assert!((test.balance()- 0.0).abs() < 1e-10);
    }
    

    //depositing and withdrawing negative money: START


    #[test]
    #[should_panic(expected = "You can't deposit negative money silly!!")]
    fn test_deposit_negative_money(){
        let mut test = BankAccount::new(0.0);
        test.deposit(-50.0)
    }

    #[test]
    #[should_panic(expected = "You can't withdraw negative money silly!!")]
    fn test_withdraw_negative_money(){
        let mut test = BankAccount::new(1.0);
        test.withdraw(-50.0)
    }

    //depositing and withdrawing negative money: END

    

    //extra withdraw panic tests: START
    #[test]
    #[should_panic(expected = "You don't have money to withdraw!!")]
    fn test_withdraw_money_empty(){
        let mut test = BankAccount::new(0.0);
        test.withdraw(10.00)
    }

    #[test]
    #[should_panic(expected = "You don't have that much money to withdraw!")]
    fn test_withdraw_extra_money(){
        let mut test = BankAccount::new(10.0);
        test.withdraw(1000.00)
    }

     //extra withdraw panic tests: END

}

