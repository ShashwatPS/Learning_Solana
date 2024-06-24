fn main() {
    // Immutable
    let num : i32 = -100;
    println!("{}",num);

    //Mutable
    let mut name = String::from("Hey how is it going on");
    println!("{}",name);
    name = String::from("Kiminowa");
    print!("{}",name);
}