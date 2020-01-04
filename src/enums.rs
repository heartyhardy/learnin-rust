pub enum Movement{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub fn move_avatar(m:Movement){
    match m{
        Movement::UP => println!("Moving Up..."),
        Movement::DOWN => println!("Moving Down..."),
        Movement::LEFT => println!("Moving Left..."),
        Movement::RIGHT => println!("Moving Right...")
    }
}

pub fn run(){
    move_avatar(Movement::UP);
    move_avatar(Movement::LEFT);
    move_avatar(Movement::DOWN);
    move_avatar(Movement::RIGHT);
}