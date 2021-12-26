pub fn run() {
    // print console
    println!("Hello, world! {}", 1);
    println!("{} {} {}", 2, 3, 4);
    println!("{:b} {:o} {:x}", 2, 3, 4);

    // debug
    println!("{:?}", (12, true, "hello"));

    // debug
    println!("{name}", name = "hello");
}
