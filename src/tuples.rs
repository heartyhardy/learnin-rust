pub fn run(){
    // Tuples
    let person: (&str, &str, i8) = ("John", "Gaming", 30);
    println!("{:?}", person);
    println!("{} loves {} and he's {} years old", person.0, person.1, person.2);
}