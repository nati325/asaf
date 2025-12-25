use crate::bank::Bank;
use crate::customer::Customer;

#[derive(Clone)]
pub struct Transaction {
    pub id: u32,
    pub amount: f64,
    pub customer: Customer,
    pub bank: Bank,
}

impl Transaction {
    pub fn print_info(&self) {
        println!("Transaction ID: {}, Amount: {}, Customer: {}, Bank: {}", self.id, self.amount, self.customer.name, self.bank.name);
    }
    
    pub fn new(id: u32, amount: f64, customer: Customer, bank: Bank) -> Self {
        Transaction { id, amount, customer, bank }
    }
}