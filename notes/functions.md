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