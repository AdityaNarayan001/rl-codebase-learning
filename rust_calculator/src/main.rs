use rust_calculator::add;

fn main() {
    println!("Simple Calculator (Only Addition)");

    let list1 = [1, 2, 3, 4, 5];
    let list2 = [10, 20, 30, 40, 50];

    for i in 0..list1.len() {
        let result = add(list1[i], list2[i]);
        println!("{} + {} = {}", list1[i], list2[i], result);
    }
}
