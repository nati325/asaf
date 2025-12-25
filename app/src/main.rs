mod bank;
mod customer;
mod account;
mod transaction;

fn main() {
    use crate::bank::Bank;
    use crate::customer::Customer;
    use crate::account::Account;
    use crate::transaction::Transaction;

    let bank = Bank::new(String::from("Madar Bank"), 1000000.0);
    bank.print_info();

    let customer = Customer::new(String::from("Alice"), 30, 1, bank.clone());
    customer.print_info();

    let account = Account::new(1, 5000.0, customer.clone(), bank.clone());
    account.print_info();

    let transaction = Transaction::new(1, 1500.0, customer.clone(), bank.clone());
    transaction.print_info();
}