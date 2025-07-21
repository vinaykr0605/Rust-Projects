
pub fn main() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary: u8 = match boolean{
        true => 1,
        false => 0
    };

    assert_eq!(binary, 1);

    println!("Success!");
}