use std::mem;

pub fn run(){
    // Vectors are resizable arrays
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", nums );

    // Push number to vector
    nums.push(6);
    println!("{:?}", nums );

    // Pop a number
    println!("{}, {:?}", nums.pop().unwrap(), nums );

    // Loop through a vector
    for num in nums.iter(){
        print!(" {}, ",num);
    }
    println!("");

    // Loop while modifying current item
    let mut lnum = 0;
    for num in nums.iter_mut(){
        *num += 1;
        lnum += *num;
    }
    println!("Mutated: {:?}", nums);
    println!("Total: {}",lnum );
    
}