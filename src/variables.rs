pub fn run(){
    let name = "John";
    let mut age = 33;
    println!(
        "My name is {} and I'm {} years old",
         name, age
    );

    // Mutating Age
    age = 37;
    println!(
        "My name is {} and I'm {} years old",
         name, age
    );

    // Constants
    const ID: i32 = 820334895;
    println!("My ID is {}",  ID);

    // Multiple variable assignment
    // Note: name and age are shadowed here
    let (name, age) = ("Joe", 54);
    println!(
        "My name is {} and I'm {} years old",
         name, age
    );
}