use std::mem;

pub fn run(){
    // Defining an array
    let nums: [i32;5] = [1,2,3,4,5];
    println!("{:?}",nums );

    // Print first element
    println!("First element is: {}",nums[0] );

    // Can reassign values of individual elements
    let mut nums: [i32;5] = [1,2,3,4,5];
    nums[0] = 10;
    println!("{:?}",nums );

    // Length of the array
    println!("Length of array nums: {}", nums.len() );

    // Memory allocation of the array
    println!("Used memory in bytes: {}", mem::size_of_val(&nums) );

    // Slices
    let slice: &[i32] = &nums;
    println!("Slice: {:?}", slice );

    // Slice a part
    let slice: &[i32] = &nums[1..3];
    println!("From 1st to 3rd {:?}", slice);
}