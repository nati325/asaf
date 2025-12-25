use crate::bank::Bank; // ייבוא המבנה Bank
use crate::costomer::Customer; // ייבוא המבנה Customer
#[derive(Clone)]
struct Account {
    id: u32,
    balance: f64,
    customer: Customer, // קישור למבנה Customer 
    bank: Bank,         // קישור למבנה Bank
}   
impl Account {
    fn print_info(&self) {
        println!("Account ID: {}, Balance: {}, Customer: {}, Bank: {}", self.id, self.balance, self.customer.name, self.bank.name);
    }
    fn new (id:u32, balance:f64, customer: Customer, bank: Bank) -> Self {
        Account { id, balance, customer, bank }
    }
}