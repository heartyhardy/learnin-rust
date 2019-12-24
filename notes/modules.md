#Learning Rust

##Modules

* To import a module use `mod <filename>`

```rust
    mod print;

    print::Run()
```
* To import a namespace, such as `std` you can...
  
  ```rust
    use std::mem;
    println!("{}", mem::size_of_val(&nums));
  ```
