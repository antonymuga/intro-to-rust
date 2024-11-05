fn main() {
    let company:&str = "RuntimeLab"; // with string literals, value is known at compile
                                                 // time
    let headquarters:&str = "Nairobi";
    println!("The organization is : {} location is :{}", company, headquarters);
}
