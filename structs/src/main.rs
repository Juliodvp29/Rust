struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let mut user1 = User {
        active: true,
        username: String::from("juliodvp29"),
        sign_in_count: 1,
        email: String::from("example@gmail.com"),
    };

    println!("Hello, world!");
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);


    let _user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // let _user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

   let  rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));

    
    
}



fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}