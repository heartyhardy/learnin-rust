// Traditional struct
struct Color{
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Coordinates(i32, i32);

// Implementing functions in structs
impl Coordinates{
    fn new(x:i32, y:i32) -> Coordinates {
        Coordinates(x, y)
    }
}

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

pub fn run(){
    let mut pure_red:Color = Color{red: 255, green:0, blue:0};
    println!(
        "Pure red: R: {} G:{} B:{}",
        pure_red.red, pure_red.green, pure_red.blue
    );

    pure_red.blue=255;
    println!(
        "Purple: R: {} G:{} B:{}",
        pure_red.red, pure_red.green, pure_red.blue
    );

    let xy:Coordinates = Coordinates(10,20);
    println!("XY coordinates ({},{})",xy.0, xy.1 );

    // To call the new function in Coordinates
    let coords = Coordinates::new(20, 50);
    println!("XY coordinates ({},{})",coords.0, coords.1 );

    // To call the new function in Person
    let mut person = Person::new("Chris", "Rook");
    println!("New Person: First name: {}, Last name: {}",person.first_name, person.last_name );
    println!("Full Name: {}", person.full_name());

    person.set_last_name("Todd");
    println!("Full Name: {}", person.full_name());

    let person_tuple = person.to_tuple();
    println!("Tuple: {} {}", person_tuple.0, person_tuple.1);

}