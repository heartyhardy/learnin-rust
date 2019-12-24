#Learning Rust

## Arrays

* Arrays in Rust are **fixed-lengthed**.
* **Data type** of **all elements** are **same**.
* There are **growable** arrays which are called **Vectors**.

```rust
    let nums: [i32; 5] = [1,2,3,4,5];
```
* Can reassign values using `mut` keyword.

```rust
    let mut nums: [i32;5] = [1,2,3,4,5];
    nums[0] = 10;
```
* Can get the **length** of the array using `len()` method.
* Can get the **memory allocation** in bytes, of the array via...
  
```rust
    std::mem::size_of_val(&nums);
```

### Slices

* Use Slices to **extract a part** of an array.
  
  ```rust
    //Copy fully
    let slice: &[i32] = &nums;

    //Copy a part
    let slice: &[i32] = &nums[1..3];
  ```