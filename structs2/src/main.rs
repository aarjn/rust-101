#[derive(Debug)]
struct User1 {
    name: String,
    place: String,
    age: u8,
}

#[derive(Debug)]
struct User2<'a> {
    name: &'a str,
    place: &'a str,
    age: u8,
}

struct Shape {
    x: u32,
    y: u32,
}

impl Shape {
    fn square(&self) -> bool {
        self.x == self.y
    }
}

fn main() {
    let user1 = User1 {
        name: String::from("pieg"),
        place: String::from("del"),
        age: 32,
    };
    //dbg!(user1);

    let user1dummy = User1 {
        name: String::from("adma"),
        ..user1
    };
    println!("user1dummy.name: {}", user1dummy.name);
    println!("user1.name: {}", user1.name);
    //println!("user1.place: {}", user1.place); // give error since those fields are moved to user1dummy

    let user2 = User2 {
        name: "pieg",
        place: "del",
        age: 32,
    };
    dbg!(user2);

    let square = Shape { x: 10, y: 10 };

    if square.square() {
        println!("Yes im square");
    }

    println!("Hello, world!");
}
