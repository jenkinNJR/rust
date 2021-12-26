pub fn run() {
    //default it is  i32
    let x = 1;

    //default it is f64
    let y = 2.5;
    //explicit type
    let z: i64 = 5342325;
    // max size
    println!("max size i32: {}", std::i32::MAX);
    println!("max size i64: {}", std::i64::MAX);

    //boolean
    let is_active: bool = true;

    // get bool from expression

    let is_grater: bool = 10 < 5;

    // string

    let ai = "adfaf";
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_grater, ai, face));
}
