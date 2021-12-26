pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    let command = args[1].clone();
    println!("{}", command);

    let name = "raj";

    if command == "hello" {
        print!("{}", name);
    }
}
