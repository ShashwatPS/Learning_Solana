fn main() {
    let s1 = String::from("First String");
    let s2 = &s1; //Refernces -> Address is passed rather than ownership
    println!("s1: {}",s1);
    println!("s2: {}",s2);

    let my_string = String::from("Hello, Rust!");
    takes_ownership(&my_string);  // Pass a reference to my_string
    println!("{}", my_string);    // This is valid because ownership was not transferred

    // Mutable Reference
    let mut s3 = String::from("Third String");
    let s4 = &mut s3;
    s4.push_str(" Added");
    println!("s3: {}",s3);
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string);  // some_string is borrowed and not moved
}