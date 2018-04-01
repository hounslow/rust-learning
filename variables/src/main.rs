// fn main() {
//     let mut x = 5;
//     // constants are ALWAYS immutable
//     // values also must be annotated
//     // must be set to a constant expression
//     // can't be set to the result of a function call
//     const Y: u32 = 6;
//     println!("{}", Y);
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

fn main() {
    let mut x = 5;
    x = x + 1;
    x = x * 2;

    println!("The value of x is: {}", x);
}
