// fn main() {
  
//     // let mut s = String::from("hello");
//     // s.push_str(", world!");
//     // println!("{}", s);

//     // let s1 = String::from("Hello");
//     // let s2 = s1;
//     // println!("{}", s2);

//     // let s1 = String::from("Hello");
//     // let s2 = s1.clone();
//     // println!("s1 = {}, s2 = {}", s1, s2);

    
  
// }

fn main() {
    // let s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1);

    // println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }


fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}