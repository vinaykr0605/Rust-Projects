// Fix the error without removing any code
pub fn main() {
    let s = String::from("Hello World");

    // print_str(s.clone());
    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}