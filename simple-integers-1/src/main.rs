fn main(){
    let rating = 10; // integer signed 32 bit by default
    let my_age:u32 = 2; //unsigned 32 bit integer
    let my_rank:i32 = 5-15; // signed 32 bit integer
    let mark:isize = 10;// architect dependent integer that is signed
    let count:usize = 30; // architecture dpendent integer that is unsigned
    println!("rating value is {}", rating);
    println!("my age is {} and rank is {}", my_age, my_rank);
    println!("I got {} marks in {} exams", mark, count);
}
