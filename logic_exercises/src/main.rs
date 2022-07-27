fn main() {
    // fibonacci(5);
    let x = fahrenheit_to_celsius(30.1);
    println!("{}", x);

}

//1. fibonacci recursive
fn fibonacci_r(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci_r(n - 1) + fibonacci_r(n - 2);
    }
}

//2. fibonacci iterative 
fn fibonacci(num: u32) {
    let n = num;
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    let mut counter = 0;

    loop{
        c = a+b;
        a = b;
        b = c;

        counter += 1;
        if counter == n{
            break;
        }

        println!("{counter}. - {c}");

    }
}


// convert farhenheit to celsius
fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

