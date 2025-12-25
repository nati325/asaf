use crate::bank::Bank; // ייבוא המבנה Bank

#[derive(Clone)]
struct Customer {
    id: u32,
    name: String,
    age: u32,
    bank: Bank, // קישור למבנה Bank
}

impl Customer {
    fn print_info(&self) {
        println!("Customer Name: {}, Age: {}, ID: {}, Bank: {}", self.name, self.age, self.id, self.bank.name);
    }

    fn new(name: String, age: u32, id: u32, bank: Bank) -> Self {
        Customer { id, name, age, bank }
    }
}