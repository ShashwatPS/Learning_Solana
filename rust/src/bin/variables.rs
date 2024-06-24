fn main() {
    //Numbers
    let x = 1;
    println!("This is the value of x : {}",x);

    //Booleans
    let male = true;
    let above_18 = true;

    if male {
        println!("Male True");
    } else if above_18  {
        println!("Above 18");
    }
    if male && above_18 {
        println!("Male and above 18\n")
    }

    //Strings
    let name = String::from("Kiminowa");
    println!("{}",name);
}