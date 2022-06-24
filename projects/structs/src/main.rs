struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

struct UserWithError {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[allow(unused)]
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername_123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("user1 email is '{}'.", user1.email);

    // struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        // active: user1.active,
        // username: user1.username,
        // sign_in_count: user1.sign_in_count,

        // The syntax `..` specified that the remaining fields
        // not explicitly set should have the same value as
        // the fields in the given instance.
        ..user1
    };
    println!("user2 name is '{}'", user2.username);

    let black = Color(0, 0, 0);
    let orange = Point(255, 165, 0);

    println!("black.0 is '{}'", black.0);
    println!("orange.1 is '{}'", orange.1);

    let subject = AlwaysEqual;

    let user3 = UserWithError {
        email: "got-an-error!@error.com".to_string(),
        username: "someusername404".to_string(),
        active: true,
        sign_in_count: 1,
    };

    println!("user3: '{}', '{}'", user3.username, user3.email);

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (40, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        // `ref` means refactoring
        ref_tuple_area(rect1)
    );

    let rect1 = Rectangle {
        width: 50,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        ref_struct_area(&rect1)
    );

    let rect1 = Rectangle {
        width: 60,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!(
        "from impl Rectangle.area:\nThe area of rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}.", rect1.width);
    }
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);

            f64::sqrt(x_squared + y_squared) // return value
        }
    }

    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };

    println!("{}", p1.distance(&p2));
    println!("{}", (&p1).distance(&p2));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("'{:?}'.", sq);
} // end of fn main()

// function area
fn build_user(email: String, username: String) -> User {
    User {
        // shorthand
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn ref_tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn ref_struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
