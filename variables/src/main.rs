fn main() {
    let mut x = 5;
    // constants are ALWAYS immutable
    const Y: u32 = 6;
    println!("{}", Y);
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
