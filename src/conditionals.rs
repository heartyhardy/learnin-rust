pub fn run(){
    // IF condition
    let x: i32 = 10;
    let y: i32 = 20;

    if x > y {
        println!("{} is Greater than {}", x, y )
    }else{
        println!("{} is Greater than {}", y, x )
    }

    //Shorthand IF, similar to ternary operator
    let is_x_greater = if x > y {true} else {false};
    println!("Is x > y? {}",is_x_greater );

}