use std::mem::size_of_val;

pub fn run() {
    //arrays are fixed

    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // reassingn value
    numbers[2] = 20;

    println!("{:?}", numbers);

    //get single val
    println!("single val {}", numbers[1]);

    //length
    println!("{}", numbers.len());

    //memory allocation

    println!("{}", size_of_val(&numbers));

    let copy: &[i32] = &numbers[0..2];
    println!("{:?}", copy);
}
