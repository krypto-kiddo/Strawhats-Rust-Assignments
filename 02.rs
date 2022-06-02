#[macro_use] extern crate text_io;
fn main(){
println!("Hello");
let n1:i32=read!(); // Weird, i cant start my fn main with a "let"
println!("The first number is : {}",n1);
let n2:i32=read!();
println!("The second number is : {}",n2);
println!("The sum of the two numbers is : {}",n1+n2);
}
