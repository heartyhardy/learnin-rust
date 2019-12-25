#Learning Rust

## Loops

* Use the `loop` keyword to **loop infinitely**.
  
  ```rust
    // Infinite loop
    let mut count: u8 = 0;

    loop{
        count += 1;
        println!("Current count: {} ",count );
        
        if count >= std::u8::MAX {
            break;
        }
    }

  ```

* Use `While <Condition>` for a while loop.
  
  ```rust
    // While loop (FizzBuzz)
    let mut count: u8 = 0;

    while count < std::u8::MAX {
        if count % 15 == 0{
            println!("\t\t\t\t\t\tFizzBuzz! @{}",count );
        }else if count % 3 == 0{
            println!("\tFizz! @{}",count );
        }else if count % 5 == 0{
            println!("\t\t\tBuzz! @{}",count );
        }
        count += 1
    }
  ```

* There's **For Range loop** `for in start..end`

  ```rust
    // For Range loop
    for x in 0..100{
        println!("{} {} {}", x , x %10 == 0, x % 10 );
    }
  ``` 