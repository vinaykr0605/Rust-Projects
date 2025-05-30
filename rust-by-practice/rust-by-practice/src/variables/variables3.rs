
// Fix the error below with least amount of modification
pub fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y); 
}