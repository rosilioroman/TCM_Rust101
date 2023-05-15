#![allow(unused)]

/* Challenge 1 - Build a program that has the following:
    1) Has a global constant integer named 'birthday' with a value of 1
    2) Has a local string variable named 'my_name' with your name as the value
    3) Has a local string variable named 'my_birthday' with your birth month/day (no year) as the value
    4) Has a local mutable integer variable named 'age' with your current age as the value
    5) Has a local integer variable named 'new_age' with your age after your birthday as the value
    6) Prints out 'My name is X and I am X years old. I will turn X on X' 

*/

    // Task 1
    const BIRTHDAY: i32 = 1;
    fn main() {
        // Task 2
        let my_name = "Rosilio";

        // Task 3
        let my_birthday = "May 6th";

        // Task 4 
        let mut age = 29;
        
        // Task 5
        let new_age = age + BIRTHDAY;

        // Task 6
        println!("My name is {} and I am {} years old. I will turn {} on {}.", my_name, age, new_age, my_birthday);
    }