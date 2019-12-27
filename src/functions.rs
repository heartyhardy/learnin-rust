pub fn run(){
    greet("Joe", "Hi there");

    // Bind function return to a variable
    let ans:i32 = add(10, 20);
    println!("Addition: {}", ans);
}

// Simple method, no return type
pub fn greet(name: &str, greeting: &str){
    println!("{}, {}", greeting, name);
}

// Add function returns a number
pub fn add(op1:i32, op2:i32) -> i32{
    op1 + op2
}

