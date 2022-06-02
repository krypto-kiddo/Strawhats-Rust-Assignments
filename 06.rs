struct Shape{
  length:i32,
  width:i32,
}
impl Shape{
  fn area(&self)->i32{
    self.length*self.width
  }
  
  fn perimeter(&self)->i32{
    2 * (self.length + self.width)
  }
}

fn main(){
  // driver code 
  let not_shape = Shape{
    length: 420,
    width: 69,
  };

  println!("The area is : {}",not_shape.area());
  println!("The perimeter is : {}",not_shape.perimeter());
  
}
