fn main(){
    //! cannot assign twice to immutable variable `x`
    
    /*
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    */

    // * "mut" keyword is used to make a variable mutable

    /* 
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    */

    // * shading

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

}

// const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;


/*
using mutcon constants is not allowed. Constants are not only immutable by default, 
they are always immutable. Constants are declared using the const keyword instead of the keyword, 
and the type of the value must be noted.
 */




