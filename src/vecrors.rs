use std::mem::size_of_val;

pub fn run() {
    //arrays are fixed

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // reassingn value
    numbers[2] = 20;

    //add on to vector

    numbers.push(78);
    numbers.push(34);

    println!("{:?}", numbers);

    //get single val
    println!("single val {}", numbers[1]);

    //length
    println!("{}", numbers.len());

    //memory allocation

    println!("{}", size_of_val(&numbers));
    numbers.pop();

    let copy: &[i32] = &numbers[0..2];
    println!("{:?}", copy);

    // loop

    for elem in numbers.iter() {
        println!("{}", elem);
    }

    //mutate loop

    for elem in numbers.iter_mut() {
        *elem *= 2;
    }

    println!("{:?}", numbers);
}
