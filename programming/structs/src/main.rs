// --- Structs ---
// a data structure that allows us to group multiple fields together under one name
// we can create our own custom data types using structs

struct BankAccount {
    account_number: String,
    balance: f64,
    owner_name: String,
}

impl BankAccount {
    fn new(account_number: String, balance: f64, owner_name: String) -> Self {
        BankAccount {
            account_number,
            balance,
            owner_name,
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrawing {} from account: {}", amount, self.account_number);
            println!("Withdrawal successful. Balance after withdrawal: {}", self.balance);

        } else {
            println!("Insufficient funds");
        }
    }

    // imutable method to check the balance
    fn check_balance(&self) -> f64 {
        println!("Checking balance for account: {}", self.account_number);
        println!(" {}", self.balance);
        self.balance
    }
}
// impl block is used to define methods for the struct, we can have multiple impl blocks for the same struct
// we can also define associated functions (static methods) for the struct, which are called using the struct name instead of an instance
// For example, we can define a function to create a new bank account without needing to specify the account number, balance, and owner name every time we want to create a new account. 
//We can use the new function defined in the impl block to create a new bank account instance.   
fn main(){
    // creating a struct instance
    let mut account = BankAccount::new("123456789".to_string(), 1000.0, "John Doe".to_string());
    println!("Account Number: {}", account.account_number);
    println!("Balance: {}", account.balance);
    println!("Owner Name: {}", account.owner_name);

    // depositing money
    account.deposit(500.0);
    println!("Balance after deposit: {}", account.balance);

    // withdrawing money
    account.withdraw(200.0);
   

    // checking balance [immutable method]
    account.check_balance();

    // trying to withdraw more than the balance
    account.withdraw(300.0);

    account.check_balance();


}
