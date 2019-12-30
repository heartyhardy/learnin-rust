pub fn run(){
    greet("Joe", "Hi there");

    // Bind function return to a variable
    let ans:i32 = add(10, 20);
    println!("Addition: {}", ans);

    //Closures
    let n3: i32 = 50;
    let add_num = |n1: i32, n2: i32|n1+n2+n3;
    println!("C Sum: {}", add_num(10, 20));

    let n: i32 = 5;
    let fac: i32 = factorial(n);
    println!("Factorial of {} is {}",n, fac );
}

// Simple method, no return type
pub fn greet(name: &str, greeting: &str){
    println!("{}, {}", greeting, name);
}

// Add function returns a number
pub fn add(op1:i32, op2:i32) -> i32{
    op1 + op2
}

pub fn factorial(n: i32) -> i32{
    if n == 1{
        return 1
    }
    n * factorial(n-1)
}

