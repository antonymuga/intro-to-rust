fn main() {
    // String::new() creates a new string obj from standard library pub struct String, it's empty
    // String::from() accepts the parameter value of created empty string
    let empty_string = String::new();
    println!("length is {}", empty_string.len());

    let content_string = String::from("Runtime lab Africa");
    println!("length is {}", content_string.len());
}
