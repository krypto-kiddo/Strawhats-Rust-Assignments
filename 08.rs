#[macro_use] extern crate text_io;

struct UserInput {
  numb: i32,
}

pub trait Error{
  fn negative_error(&self);
}

impl Error for UserInput{
  fn negative_error(&self) {
    if self.numb<0{
      println!("ERROR : Negative Number");
    }
  }
}

fn main(){
  let ui = UserInput{
    numb: read!(),
  };
  ui.negative_error();
}
