#Learning Rust

##Strings

There are two types of **strings**

* **Primitive strings**
  >Primitive strings are immutable and stored in memory somewhere.

  ```rust
    // Immutable primitive string
    let greet = "Hello";
  ```

* **String**
  >Growable and stored in heap-allocated data structure. Use this when you need to modify or own string data.

  ```rust
    // Growable
    let greet = String::from("Hello");
  ```
    * To ***Push a character*** to a growable string...
    
    ```rust
        // Pushing a char
        greet.push('\u{1F601}');
    ```
    * To ***Push a string*** ...
  
    ```rust
        greet.push_str("  there! Let's have a drink! \u{1F37A}");
    ```
