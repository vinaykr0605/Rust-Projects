
// Fix the errors without adding or removing lines
pub fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in &names {
        // Do something with name...
        println!("{}",name);
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with n...
        println!("{}",n);
    }
    
    println!("{:?}", numbers);
} 