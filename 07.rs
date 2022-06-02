fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}",s); // Added print
}

fn change(some_string: &mut String) {
    some_string.push_str(", Solana");
}
