pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;

    if age >= 21 && check_id {
        println!("you are greater than {}", age);
    } else if age < 21 && check_id {
        println!("get out");
    } else {
        println!("show id");
    }

    // shorthqand if

    let is_of_age = if age >= 21 { true } else { false };

    println!("{}", is_of_age);

    println!("{}", add(2, 3));

    //closure

    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    print!("{}", add_nums(1, 1));
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
