fn main() {
    let number = 9;

    if number % 2 == 0 {
        println!("{} is even", number);
    }else{
        println!("{} is odd", number);
    }

    let number2 = 9;
    if number != number2 {
        println!("{} is not equal to {}", number, number2);
    }else if number == number2 {
        println!("{} is equal to {}", number, number2);
    }else{
        println!("Goodbye!");
    }

    let condition = false;
    let number3 = if condition {5} else {6};
    println!("The value of number3 is: {}", number3);

    if 2 >= 1 {
        println!("2 is greater than or equal to 1");
    }

    if "hello" == "world" {
        println!("hello is equal to world");
    }else if "hello" == "hello" {
        println!("hello is equal to hello");
        
    }

    let condition2 = false;
    let number4 = if condition2 {5} else {6};
    println!("The value of number4 is: {}", number4);
}
