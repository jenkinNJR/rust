// used to create custom data types

//tradisonal struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct

struct Colors(u8, u8, u8);

struct Person {
    f_n: String,
    ln: String,
}

impl Person {
    //construct person
    fn new(f: &str, l: &str) -> Person {
        Person {
            f_n: f.to_string(),
            ln: l.to_string(),
        }
    }

    // full name

    fn full_name(&self) -> String {
        format!("{} {}", self.f_n, self.ln)
    }

    fn setlastname(&mut self, last: &str) {
        self.ln = last.to_string();
    }

    // name to tuple

    fn to_tuple(self) -> (String, String) {
        (self.f_n, self.ln)
    }
}

pub fn run() {
    let mut c = Color {
        red: 12,
        green: 34,
        blue: 23,
    };

    c.red = 200;

    println!("{} {} {}", c.blue, c.green, c.red);

    let mut c2 = Colors(1, 2, 3);
    c2.2 = 70;
    println!("{} {} {}", c2.1, c2.2, c2.0);

    let mut p = Person::new("sam", "doe");

    println!("{} {}", p.f_n, p.ln);
    p.setlastname("ddddd");
    println!("{}", p.full_name());
    println!("{:?}", p.to_tuple());
}
