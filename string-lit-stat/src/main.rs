fn main() {
    // string literals are static by nature, guranteed not to change in entire program
    // can also be explicitly declared as static
    let company:&'static str = "RuntimeAnalytica";
    let location:&'static str = "Nairobi";
    println!("Org name is : {} and location is: {}", company, location);
}
