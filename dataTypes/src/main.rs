fn main() {

    //Data Types
    //Scalar
    let num1: u128 = 90; // Here u stands for unsigned Integers and 128 is basically bits
    let num2: i128 = 90; // Here i is Basically for signed Integers
    let x = 2.0; //By default it is f64
    let y:f32 = 3.0; // Float only has 2 types that is f32 and f64
    let verdict: bool = true; //Boolean Type
    let z: char = 'Z'; // Can also put an Emoji Here
    let emo = '🥰';

    //Compounds
    let tup: (i32, f64, u8) = (500, 6.4,1); //This is a Tuple
    let (x,y,z) = tup;
    println!("{x}");

    let x: (i32, f64, u8) = (500, 6.4,1); // This is Another way to print Tuple
    let fiveHundred = x.0;
    println!("{fiveHundred}");



    //Numeric Operations:
    let sum = 5+4;
    let difference = 5-4;
    let product = 4*30;
    let quotient = 4/2;
    let remainder = 43%5;
}
