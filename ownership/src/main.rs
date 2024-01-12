fn main() {
    variable_scope();
    string_type();
}

fn variable_scope() {
    println!("#### variable scope ####");
    {
        // s is not valid here, it's not yet declared
        let s = "hello"; // s is from this point forward
        println!("{s}");
    } // this scope is over now, and s in no longer valid
    println!("\n");
}

fn string_type() {
    println!("#### string type ####");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    println!("\n");
}

fn variable_data_move() {
    let s1 = String::from("hello"); // create s1 in function scope
    let s2 = s1; // move to ownership from s1 to s2, now s2 is no longer valid
    println!("{s1}");
    // for simple data type, the ownership is not moved but copied
}

fn function_ownership(s: String) -> usize {
    s.len() // the string argument is moved
} // after goes out of function scope, string value no longer exist

fn function_borrowing(s: &String) -> usize {
    // Now the parameter is passed by reference (the function
    // borrow the variable, and no use drop at the end of function scope)
    s.len()
}
