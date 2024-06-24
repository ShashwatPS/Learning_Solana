use crate::utils::get_first_name;

//Modules
mod utils;

fn main() {
    let num: u32 = 45;
    let chk : bool = utils::is_even(num);
    if chk {
        print!("The number is even")
    } else {
        print!("The number is Odd")
    }

    let firstName = utils::get_first_name(String::from("shashwat pratap singh"));
    print!("The first name is: {}",firstName);
}