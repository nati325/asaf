fn main() {
   let mut vec = vec![1, 2, 3, 4, 5,6];
    let result = sum_even_numbers(&vec);
    println!("The sum of even numbers is: {}", result);    Start-Process "https://aka.ms/vs/17/release/vs_buildtools.exe"
}
fn sum_even_numbers(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for &num in numbers.iter() {
            sum += num;
        
    }
    sum
}
