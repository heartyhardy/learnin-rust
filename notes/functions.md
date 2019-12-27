#Learning Rust

## Functions

* Use `->` followed by the **data type** to specify the return value.
* **To **return**, **don't use a semicolon**.
  
```rust
    // Add function returns a number
    pub fn add(op1:i32, op2:i32) -> i32{
        op1 + op2
    }
```
* To define a closure, use `pipe |` as following...
  
```rust
    //Closures
    let add_num = |n1: i32, n2: i32|n1+n2;
    println!("C Sum: {}", add_num(10, 20));
```