pub fn run(){
    // Primitive string
    let _greet = "Hello";

    // Growable
    let greet = String::from("Hello");
    println!("{} there! How are you?", greet );

    // Length of the string
    println!("Length: {}", greet.len() );

    // Capacity of the string
    println!("Capacity: {}", greet.capacity());

    // Is Empty?
    println!("Is Empty: {}", greet.is_empty());

    // Contains substring
    println!("Contains Hello ?: {}", greet.contains("Hello"));

    // Replace a substring
    println!("{} there! How are you?", greet.replace("Hello","Hey") );

    // Mutating a growable string
    let mut greet = String::from("Hello ");
    // Pushing a char
    greet.push('\u{1F601}');
    println!("{}  there! How are you?", greet );

    // Pushing a string
    greet.push_str("  there! Let's have a drink! \u{1F37A}");
    println!("{}", greet );

    // Looping through a string
    println!("");
    let mut count = 0;
    for word in greet.split_whitespace(){
        count = count+1;
        println!("{}. {}", count, word );
    }

    // Creating a string with capacity
    let mut new_greet = String::with_capacity(100);
    for word in greet.split_whitespace(){
        new_greet.push(' ');
        new_greet.push('\u{1F539}');
        new_greet.push(' ');
        new_greet.push_str(word);
    }
    println!("{}", new_greet );

    // String assertions
    // Will only output to console if failed
    assert_eq!(greet, new_greet);

}