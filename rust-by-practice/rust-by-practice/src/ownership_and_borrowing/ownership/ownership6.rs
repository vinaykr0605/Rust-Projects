
// make the necessary variable mutable
pub fn main() {
    let s = String::from("Hello ");
    
    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}