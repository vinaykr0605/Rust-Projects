 // Example
//  pub fn main() {
//     let x = 5u32;

//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;

//         // This expression will be assigned to `y`
//         x_cube + x_squared + x
//     };

//     let z = {
//         // The semicolon suppresses this expression and `()` is assigned to `z`
//         2 * x
//     };

//     println!("x is {:?}", x); // 5
//     println!("y is {:?}", y); // 155
//     println!("z is {:?}", z); // 10
// }

// Make it work with two ways
pub fn main() {
   let v = {
       let mut x = 1;
       x += 2;
       x
   };

   assert_eq!(v, 3);

   println!("Success!");
}