fn main() {

    /* 
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    let z = -12; // i32
    let w = -12.0; // f64
    let a = 256; // u8

    println!("x = {}, y = {}, z = {}, w = {}, a = {}", x, y, z, w, a);
    */

    //addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; 
    println!("floored = {}", floored);

    // remainder
    let remainder = 43 % 5;

    //boolean
    let t = true;
    let f: bool = false;

    //char
    let c = 'z';
    let z:char = 'ℤ';
    let heart_eyed_cat = '😻';
    
    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1 );
    let tup2 = (500, 6.4, 1);
    let (t1, t2, t3) = tup2;
    
    println!("The value t3 is: {t3}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {five_hundred}");

    //arrays
    //let a:[i32, 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let b = [3; 5]; // [3, 3, 3, 3, 3]
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {first}");
}

/* 
Every value in Rust is of a certain data type, which tells Rust what kind of data is being 
specified so it knows how to work with that data. We’ll look at two data type subsets: 
scalar and compound.

!Scalar types
A scalar type represents a single value. 
Rust has four primary scalar types: integers, 
floating-point numbers, Booleans, and characters. 
You may recognize these from other programming languages. 
Let’s jump into how they work in Rust.

!Integers types


Length	Signed	Unsigned
8-bit	i8	      u8
16-bit	i16	      u16
32-bit	i32	      u32
64-bit	i64	      u64
128-bit	i128	  u128
arch	isize	  usize


*/
