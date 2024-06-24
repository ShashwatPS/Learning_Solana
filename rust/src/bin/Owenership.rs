fn main() {
    // Heap Variables only like to have a single Owner
    let s1 : String = String::from("Initial String");
    let s2 = s1;
    println!("{}",s2);  // Will Work
    // print!("{}",s1); // Will give error on compile time
    //This wont work as the ownership is being tranferred to s2 from s1


    // Another way of transferring Ownership
    let s3 = String::from("Some String");
    only_take_ownership(s3);
    //Here the ownership of the current variable to the variable inside and destroyed after
    //the function exits. To fix this.
    let mut s4 = String::from("Fourth String");
    s4 = take_ownership_return(s4);
    print!("{}",s4);
    // Here the ownership is returned back from the function

    // Fix to the above issue passing variable without transferring ownership
    let s5 = String::from("Fifth String");
    dont_take_ownership(s5.clone());
    println!("{}",s5);
    // Here i can print s5 because the ownership is not transferred, a copy is.


}

fn only_take_ownership(my_string: String){
    print!("{}",my_string);
}

fn take_ownership_return(my_string: String)-> String{
    print!("{}",my_string);
    my_string
}

fn dont_take_ownership(my_string:String){
    print!("{}",my_string);
}