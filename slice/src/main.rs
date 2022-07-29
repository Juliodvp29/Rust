fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello world");
    first_word(&s);
    s.clear();

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];
    assert_eq!(_slice, &[2, 3]);
}



fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}