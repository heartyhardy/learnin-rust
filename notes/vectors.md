#Learning Rust

##Vectors

>Vectors are **resizable arrays**.

* To define a Vector use...

```rust
    let nums: Vec<i32> = vec![1,2,3,4,5];
```
* To **add an element**, use `push()` method.

```rust
    let mut nums: Vec<i32> = vec![1,2,3,4,5];
    nums.push(6);
```
* To **remove an element from rear** use `pop()` method.

```rust
    println!("{}, {:?}", nums.pop().unwrap(), nums );
```