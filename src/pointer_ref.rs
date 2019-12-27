// Reference pointers

pub fn run(){

    // With primitive types, numscopy gets a copy of the array
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    let numscopy = nums;
    println!("{:?}", (nums, numscopy) );


    // With vectors, if you assign a variable to a vector, 
    // it holds the address of the original vector
    let numsvec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let numscopy = &numsvec;
    println!("{:?}", (&numsvec, numscopy) );
}