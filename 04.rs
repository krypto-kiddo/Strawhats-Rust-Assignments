use rand::{self, Rng};
fn main() {
    let sky = vec!["cloudy", "sunny", "rainy"];
    let temperature = vec!["warm", "cold", "freezing"];
    let x = ( sky[rand::thread_rng().gen_range(0..sky.len())]   ,  temperature[rand::thread_rng().gen_range(0..temperature.len())]);

     // TODO Match the sky and temperature and print the appropriate message
     /*
        cloudly + warm = It's cloudly but it's warm
        cloudly + cold = Plesant weather
        cloudly + freezing =It's freezing outside
        sunny + warm = It's hot sunny day
        sunny + cold = It's a cold sunny day
        rainy + warm = It's raining
        rainy + cold = It's raining and it's cold
        rainy + freezing = It's raining and it's freezing
        others = Not sure what to do
     */

    match x {
      ("cloudy","warm") => println!("Its cloudy but its warm"),
      ("cloudly","cold") => println!("Plesant weather"),
      ("cloudly" , "freezing") => println!("It's freezing outside"),
      ("sunny","warm") => println!("It's hot sunny day"),
      ("sunny","cold") => println!("It's a cold sunny day"),
      ("rainy","warm") => println!("It's raining"),
      ("rainy","cold") => println!("It's raining and its cold"),
      ("rainy","freezing") => println!("It's raining and its freezing"),
      ("never","gonna") => println!("give you up, never gonna let you down."),
      _ => println!("not sure what to do?")
    }
}
