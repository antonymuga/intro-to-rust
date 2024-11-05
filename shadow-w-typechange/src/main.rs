fn main() {
    let motto = "Connect, Discover, Impact"; // original string var
    let motto = motto.len(); // shadowed var that evaluates to integer
    println!("The motto changed to an integer value: {}", motto);
}
