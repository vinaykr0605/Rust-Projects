
// Fill in the blank
pub fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    assert_eq!(result, 10);

    println!("Success!");
}