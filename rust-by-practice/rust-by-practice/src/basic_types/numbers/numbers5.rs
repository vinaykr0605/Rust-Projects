
// Fix errors and panics to make it work
pub fn main() {
   let v1 = 251_u16 + 8_u16;
   let v2 = i32::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}