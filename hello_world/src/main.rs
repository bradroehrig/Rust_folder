fn main() {

    let mut text: String = String::new();

    std::io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line.");

    println!("Value given by user is {}", text);
}