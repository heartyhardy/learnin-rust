#Learning Rust

##Types

* **Integers** -`u8, i8, u16, i16, u32, i32, u64, i64, u128, i128` (number of bytes taken)
* **Floats** - `f32, f64`
* **Boolean** - `bool`
* **Characters** - `char`
* **Tuples**
* **Arrays**
  
>Rust is a staticly typed language. It must know all the types of the variables at the compile time.

>However, compiler can usually **infer** what type what we want to use depending on value and how we use it.

```rust
    // x will be i32
    let x = 10

    // y will be f32
    let y = 2.5
```

* Use underscore to ingore unused vars warning.

```rust
    let _z: i32 = 100
```