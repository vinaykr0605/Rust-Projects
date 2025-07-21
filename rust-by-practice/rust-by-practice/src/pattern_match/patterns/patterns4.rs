
// Fill in the blank to make the code work, `split` MUST be used
pub fn main() {
    let num : Option<i32> = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}