fn main() {
    let salary = 18000;
    let salary = 82000; // shadow of original var, will compile with a warning
    println!("Your projected salary is Kshs. {}", salary);
}
