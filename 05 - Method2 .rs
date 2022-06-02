// Method 2 : Cloning variable on every reference

fn print_country(name: String) {
    println!("This is {}", name);
}

fn main() {
    // TODO - Assign a value to a variable
    let name = String::from("South Korea");
    // TODO - Use References to print name as many times as we do
    print_country(name.clone()); // Call a function with a variable as an argument
    print_country(name.clone()); // Call a function with a variable as an argument again
}
