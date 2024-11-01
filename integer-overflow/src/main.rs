fn main() {
    let age:u8 = 255;//allowed for this size
    let weight:u8 = 256; // overflow of 0
    let height:u8 = 257; // overflow of 1
    let score:u8 = 258; // overflow of 2

    println!("Hello, world!");
    println!("age is {}, weight is {}, height {}, score is {}", age, weight, height, score );
}
