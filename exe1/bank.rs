#[derive(Clone)]
struct Bank{
    name: String,
    balance: f64,
}
impl Bank {
    fn print_info(&self) {
        println!("Bank Name: {}, Balance: {}", self.name, self.balance);
    }
    fn new (name: String, balance: f64) -> Self {
        Bank { name, balance }
    }
}