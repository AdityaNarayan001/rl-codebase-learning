use rust_calculator::{add, multiply, divide};

fn main() {
    println!("Simple Calculator");

    let list1 = [1, 2, 3, 4, 5];
    let list2 = [10, 20, 30, 40, 50];

    for i in 0..list1.len() {
        let a = list1[i];
        let b = list2[i];

        // Addition
        println("Addition :");
        let sum = add(a, b);
        println!("{} + {} = {}", a, b, sum);

        // Multiplication
        println("Multiplication :");
        let product = multiply(a, b);
        println!("{} * {} = {}", a, b, product);

        // Division
        println("Divison :");
        match divide(a, b) {
            Some(quotient) => println!("{} / {} = {}", a, b, quotient),
            None => println!("{} / {} = Division by zero is not allowed", a, b),
        }
    }
}
