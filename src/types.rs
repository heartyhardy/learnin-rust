pub fn run(){
    // x will be i32
    let _x = 5;

    // y will be f32
    let _y= 5.5;

    // explicit type
    let _z: i32 = 5;

    // Max values
    println!("Max u8 is {}",std::u8::MAX );
    println!("Max u16 is {}",std::u16::MAX );
    println!("Max i32 is {}",std::i32::MAX );
    println!("Max i64 is {}",std::i64::MAX );
    println!("Max f32 is {}",std::f32::MAX );

    // Min values
    println!("Min u8 is {}",std::u8::MIN );
    println!("Min u16 is {}",std::u16::MIN );
    println!("Min i32 is {}",std::i32::MIN );
    println!("Min i64 is {}",std::i64::MIN );
    println!("Min f32 is {}",std::f32::MIN );

    // Boolean
    let _is_active = true;

    // Boolean from expression
    let is_active = 10 > 5;
    println!("Is Active? {:?}",is_active );

    // char (Unicode)
    let emoji: char = '\u{1F42F}';
    println!("Tiger: {:?}", emoji);
}