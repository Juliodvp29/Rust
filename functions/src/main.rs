fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    another_function();
    function_with_parameters(5, 2);
    greet_user("Juan");
    let five = five();
    let add_two = add_two(2);
    println!("The value of five is: {}", five);
    println!("The value of add_two is: {}", add_two);
}

fn another_function(){
    println!("Another function.");
}

fn function_with_parameters(x: i32, y: i32){
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

fn greet_user(nombre: &str){
    println!("Hola {}", nombre);
}

fn five() -> i32 {
    5
}

fn add_two(x: i32) -> i32 {
    x + 2
}