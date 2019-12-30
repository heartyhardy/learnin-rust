#Learning Rust

##Pointers

* If you assign a variable to a primitive array it will copy it.
* However, if you assign a Vector to a variable it will create a pointer reference.
  
  ```rust
    // With primitive types, numscopy gets a copy of the array
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    let numscopy = nums;
    println!("{:?}", (nums, numscopy) );


    // With vectors, if you assign a variable to a vector, 
    // it holds the address of the original vector
    let numsvec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let numscopy = &numsvec;
    println!("{:?}", (&numsvec, numscopy) );
  ```