pub fn run(){
    // Basic printing
    println!("Hello from Print.rs");

    // Basic formatting
    println!("Started learning Rust on: {}rd {} {}", 23, "December", 2019);

    // Positional arguments
    println!(
        "{0} loves {1}. Be like {0}",
        "Bill", "coding"
    );

    // Named arguments
    println!(
        "{name} is {age} years old",
        name="John", age=30
    );

    // Placeholder Traits
    println!(
        "Binary: {:b} Hex: {:x} Octal: {:0}",
        10, 10, 10 
    );

    // Debug Trait
    println!("{:?}", ("Hello", true, 101) );

    // Basic Math
    println!("10 + 15 = {}", 10 + 15 )
}