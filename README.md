# Rust Learning

## Cargo
- Package manager similar to NPM
- Can `build` your code, aka compile your code in `src/` into a `target/` folder
-

## Variables
- Immutable by default (can be made mutable by using `let mut x = 5`)
- Constants are always immutable, and must be annotated (e.g `const Y: u32 = 5`)
- You can 'shadow' variables
    ```fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}```
