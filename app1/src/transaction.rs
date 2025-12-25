use crate::bank::Bank; // ייבוא המבנה Bank
use crate::costomer::Customer; // ייבוא המבנה Customer
use crate::acount::Account; // ייבוא המבנה Account
#[derive(Clone)]
struct Transaction {
    id: u32,
    amount: f64,
    customer: Customer, // קישור למבנה Customer
    bank: Bank,         // קישור למבנה Bank
}
impl Transaction {
    fn print_info(&self) {
        println!("Transaction ID: {}, Amount: {}, Customer: {}, Bank: {}", self.id, self.amount, self.customer.name, self.bank.name);
    }
    fn new (id:u32, amount:f64, customer: Customer, bank: Bank) -> Self {
        Transaction { id, amount, customer, bank }
    }   
}