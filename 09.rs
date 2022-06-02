use rand::{self,Rng};
use text_io::read;

fn main() {
    let n1: i8 = rand::thread_rng().gen_range(0,100);
    println!("Guess the number!");
    let mut user_guess:i8 = -1;

    while user_guess!=n1{
      user_guess = read!();
      if user_guess == n1{
        println!("Congrats, you guessed it!")
      }
      else {
        println!("Uh oh, try again!");
      }
    }
}
