#Learning Rust

## Structs

>Structs are custom data types that you can create.

for example, if you want to create RGB values, its really convenient to use a struct in this case.

```rust
    struct Color{
        red: u8,
        green: u8,
        blue: u8
    }
```

to create a color...

```rust
let pure_red:Color = Color{red: 255, green:0, blue:0};
```

There are **two types of structs**
1. Traditional structs
2. Tupal structs

You can create a **Tuple struct** as following...

```rust
    struct Coordinates(i32, i32);

    // To use it
    let xy:Coordinates = Coordinates(10,20);
    println!("XY coordinates ({},{})",xy.0, xy.1 );
```
* Instead of using name you can use index when it comes to tuple structs

You can **define functions** inside Structs.

```rust
impl Coordinates{
    fn new(x:i32, y:i32) -> Coordinates {
        Coordinates(x, y)
    }
}

// To call the new function in Coordinates
let coords = Coordinates::new(20, 50);
```
Another example...

```rust

// Person struct
struct Person{
    first_name:String,
    last_name:String
}

impl Person{
    fn new(first:&str, last:&str) -> Person {
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }
    
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

// To call the new function in Person
let mut person = Person::new("Chris", "Rook");
println!("New Person: First name: {}, Last name: {}",person.first_name, person.last_name );
println!("Full Name: {}", person.full_name());

// Change the last name
person.set_last_name("Todd");
println!("Full Name: {}", person.full_name());

// Get as Tuple
let person_tuple = person.to_tuple();
println!("Tuple: {} {}", person_tuple.0, person_tuple.1);

```