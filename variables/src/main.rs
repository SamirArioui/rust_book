fn main() {
    shadowing();
    integer();
    numeric_operator();
}

fn shadowing() {
    let x = 5;
    let x = x + 1; // shadowing x
    {
        // shadowing in scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    } // inner x is destroyed when scope is over
    println!("The value of x is: {x}");
}

fn integer() {
    let x = 0xff;
    println!("The value of x is: {x}");
}

fn numeric_operator() {
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let product = 4 * 3;
    let qutient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results -1
    let reminder = 43 % 5;
}
