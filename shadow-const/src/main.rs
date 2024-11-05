fn main() {
    const COMMENT:&str = "Why are you viewing my Github?";
    const COMMENT:usize = COMMENT.len(); //shadow copy that changes the data type of a const
    println!("The comment const changed to integer: {}", NAME); //will have runtime errors
}
