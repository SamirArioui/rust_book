fn main() {
    single_if();
    multiple_if();
    if_with_let();
}

fn single_if() {
    println!("#### Single if condition ####");
    let number = 7; // conditions must a bool type not like python
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    println!("\n");
}

fn multiple_if() {
    println!("#### multiple if condition ####");
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    println!("\n");
}

fn if_with_let() {
    println!("#### using if with let ####");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    println!("\n");
}
