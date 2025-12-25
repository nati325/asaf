use crate::bank::Bank;
use crate::customer::Customer;

#[derive(Clone)]
pub struct Account {
    pub id: u32,
    pub balance: f64,
    pub customer: Customer,
    pub bank: Bank,
}

impl Account {
    pub fn print_info(&self) {
        println!("Account ID: {}, Balance: {}, Customer: {}, Bank: {}", self.id, self.balance, self.customer.name, self.bank.name);
    }
    
    pub fn new(id: u32, balance: f64, customer: Customer, bank: Bank) -> Self {
        Account { id, balance, customer, bank }
    }
}