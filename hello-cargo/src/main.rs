fn main() {
    println!("Hello, world!");
    f1();
    f2();
    f3();
    f4();
    f5();
    f6();
    f7();
}

fn f1() {
    // Declare a variable
    let a_number;

    // Declare a second variable and bind the value
    let a_word = "Ten";

    // Bind a value to the first variable
    a_number = 10;

    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);
}

fn f2() {
    // The `mut` keyword lets the variable be changed
    let mut a_number = 10;
    println!("The number is {}.", a_number);

    // Change the value of an immutable variable
    a_number = 15;
    println!("Now the number is {}.", a_number);
}

fn f3() {
    // Declare first variable binding with name "shadow_num"
    let shadow_num = 5;

    // Declare second variable binding, shadows existing variable "shadow_num"
    let shadow_num = shadow_num + 5;

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2;

    println!("The number is {}.", shadow_num);
}

fn f4() {
    let number: u32 = 14;
    println!("The number is {}.", number);
}

// let number_64 = 4.0;      // compiler infers the value to use the default type f64
// let number_32: f32 = 5.0; // type f32 specified via annotation

fn f5() {
    // Addition, Subtraction, and Multiplication
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
}

fn f6() {
    // Declare variable to store result of "greater than" test, Is 1 > 4? -- false
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);
}

fn f7() {
    let uppercase_s = 'S';
    let lowercase_f = 'f';
    let smiley_face = '😃';

    println!("{}", uppercase_s);
    println!("{}", lowercase_f);
    println!("{}", smiley_face);
}
