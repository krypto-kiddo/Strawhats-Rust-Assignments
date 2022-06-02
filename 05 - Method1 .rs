// In this instead of referencing the variable we can refer to the pointer instead everytime

fn print_country(ptr: &String) {
    println!("This is {}", ptr);
}

fn main() {
    // TODO - Assign a value to a variable
    let name = String::from("India");
    // TODO - Use References to print name as many times as we do
    print_country(&name); // Call a function with a variable as an argument
    print_country(&name); // Call a function with a variable as an argument again
}



