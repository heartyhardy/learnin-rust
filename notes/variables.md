#Learning Rust

##Variables

* Variables in Rust are **immutable** by default.
* Variables are **block scoped** in Rust.

###Mutating

* To mutate a variable use `mut` keyword.
  
  ```rust
    let mut x = 50;
    x = 25;
  ```

###Constants

* To define a constant, use `const` keyword.
  
  ```rust
    const ID: i32 = 2330367;
  ```

###Shadowing

* To Shadow a variable, redefine it in the block scope.

```rust
    let x = 100
    let x = "Hey"
```