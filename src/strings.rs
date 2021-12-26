pub fn run() {
    //primitive string
    let mut hello = String::from("Hello ");

    //get length

    println!("length: {}", hello.len());

    hello.push('w');

    //capacity in bytes
    println!("capacity {}", hello.capacity());

    //check is empty
    println!("is empty {}", hello.is_empty());

    //loop
    for elem in hello.split_whitespace() {
        println!("{}", elem);
    }

    //create with capacity

    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('e');

    // assert test
    assert_eq!(2, s.len());

    println!("{}", hello);
}
