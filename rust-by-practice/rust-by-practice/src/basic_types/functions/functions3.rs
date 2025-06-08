use core::panic;

// Solve it in two ways
// DON'T let `println!` work
pub fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    // todo!("making this not to return anything");
    panic!("I'll never return");
    
}