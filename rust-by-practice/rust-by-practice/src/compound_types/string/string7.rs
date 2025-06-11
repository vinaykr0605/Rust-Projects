// Fix error with at least two solutions
pub fn main() {
    let s = "hello, world".to_string(); // or: let s = "hello, world".to_owned();, or: let s =
                                        // String::from("hello, world");
    greetings(s.to_string())
}

fn greetings(s: String) {
    println!("{}", s)
}