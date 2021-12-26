enum Movement {
    //variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // perform action depend on info

    match m {
        Movement::Up => println!("up"),
        Movement::Down => println!("Down"),
        Movement::Right => println!("Right"),
        Movement::Left => println!("Left"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    // let avatar2 = Movement::Down;
    // let avatar3 = Movement::Right;
    // let avatar4 = Movement::Up;

    move_avatar(avatar1);
}
