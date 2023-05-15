// Comments in rust are done with 2 /
/*
This is how you do multiline comments! Similar to JS
 */

fn main() {
    // COMPOUND TYPES - Tuples and Arrays

    //tuples can hold multiple values of any type
    //max value of tuple is 12
    let student_a = ("Rosilio", 'A', 3.58);

    //you can then use the data in each index in other places (remember index starts at 0)
    // let name_student_a = student_a.0;
    // let grade_student_a = student_a.1;
    // let gpa_student_a = student_a.2;

    // you can refactor the 3 lines of code above into one line
    let (name_student_a, grade_student_a, gpa_student_a) = student_a;
    
    println!("My name is {}. My class grade is {}. My overall GPA is {}", name_student_a, grade_student_a, gpa_student_a);
}

/* RUST101 COURSE NOTES:
    println!("Hello, world!"); // This prints to stdout with a newline character at the end
    print!("Hello, world again... this time with print!()"); // This prints to stdout (with no newline)

    // Scalar types: int, float, boolean, char
    // Unsigned - never negative - u8, u16, u32, u64, u128, usize
    // Signed - can be negative or positive - i8, i16, i32, i64, i128, isize

    println!("Max size of a u32: {}", u32::MAX);
    println!("Max size of a u64: {}", u64::MAX);
    println!("Max size of a i32: {}", i32::MAX);
    println!("Max size of a i64: {}", i64::MAX);

    // Floats - f32, f64
    // what is a float? think decimals. - 3.14126
    // you need a number in front for a valid float. i.e., .14 is invalid, but 0.14 is valid

    println!("Max size of a f32: {}", f32::MAX);
    println!("Max size of a f64: {}", f64::MAX);

    // Boolean - true or false - bool
    // truth tables

    // Character - char - 4 bytes
    println!("{}", 'A');
    
    //function names should be in snake case (lowercase with _ in between)

    // Variables are immutable by default. Have to use the 'mut' keyword to make them mutable (i.e., able to be changed).
    let mut hello = "Hello, World!";
    println!("{}", hello);

    hello = "Hello, again!";
   
    println!("{}", hello);
    println!("The constant number is: {}", NUMBER);

    //Constants can be global variables. Have to be in SCREAMCASE. Have to define its type.
    const NUMBER: i32 = 17;

 */