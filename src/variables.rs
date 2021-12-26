pub fn run() {
    let name = "raj";
    let mut age = 37;
    println!("name is {} age {}", name, age);

    age = 50;
    println!("name is {} age {}", name, age);

    //const

    const ID: i32 = 50;

    println!("id : {}", ID);

    // mul var

    let (myname, myage) = ("raj", 60);
    println!("name {} age {}", myname, myage);
}
