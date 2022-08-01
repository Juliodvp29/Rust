fn main() {
    fibonacci_r(5);
    fibonacci(5);
    let x = fahrenheit_to_celsius(30.1);
    println!("{}", x);
    largest(&[1,2,3,4,5,6,7,8,9,10]);
    sum_pairs(&[1,2,3,4,5,6,7,8,9,10], &[1,2,3,4,5,6,7,8,9,10]);
    factorial(5);
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
    let mut _c = 0;
    let mut counter = 0;

    loop{
        _c = a+b;
        a = b;
        b = _c;

        counter += 1;
        if counter == n{
            break;
        }

        println!("{counter}. - {_c}");

    }
}


//3. convert farhenheit to celsius
fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}



//4. find the largest number in an array of numbers

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}


//5. create a function that takes two arrays of the same size and returns another one with the sum of the pairs of the previous ones

fn sum_pairs(list1: &[i32], list2: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..list1.len() {
        if list1[i] % 2 == 0 || list2[i] % 2 == 0 {
            result.push(list1[i] + list2[i]);
        }
    }
    print!("{:?}", result);
    result
}


// 6. factorial of a number

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}



