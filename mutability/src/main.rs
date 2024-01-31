fn main(){
    let x = 5;                      // This is Immutable for safety in Rust by default
    println!("The value of x is: {x}");
    let mut y = 7;                  // This is how to create a mutable variable in Rust
    println!("The value of y is: {y}");
    y = 8;
    println!("The value of y is: {y}");
    const  NUM :i32= 8;
    println!("The value of NUM is: {NUM}");
}