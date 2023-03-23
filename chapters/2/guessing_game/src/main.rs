/* By default, Rust has a set of items defined in the standard library that it brings into the
 * scope of every program. This set is called prelude, and you can see everything in it in the
 * standard library documentation. The prelude is a small set of items that are used so often.
 * List of preludes:
 * std::marker::{Copy, Send, Sized, Sync, Unpin},
 * std::ops::{Drop, Fn, FnMut, FnOnce},
 * std::cmp::{PartialEq, PartialOrd, Eq, Ord},
 * std::clone::Clone,
 * std::mem::drop,
 * std::convert::{AsRef, AsMut, Into, From},
 * std::borrow::ToOwned,
 * std::string::{String, ToString},
 * std::boxed::Box,
 * std::vec::Vec,
 * std::option::Option::{self, Some, None},
 * std::result::Result::{self, Ok, Err},
 * std::iter::FromIterator,
 * std::convert::{TryFrom, TryInto},
 * std::default::Default,
 * std::iter::{Iterator, Extend, IntoIterator, DoubleEndedIterator, ExactSizeIterator}
 */

use rand::Rng;
use std::cmp::Ordering; // Generate an ordering.
use std::io; // std::io is not in the prelude.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new(); // Storing values with variables. mut means variable's value can
                                       // be changed. String::new() creates a new, empty instance of a
                                       // String. In full, this line has created a mutable variable
                                       // that is currently bound to a new, empty instance of a `String`.

        /* Receiving User Input
         * If we hadn't imported the io library with std::io;, this changes into std::io::stdin().
         * .real_line(&mut guess) calls the read_line method on the standard input handle to get input
         * from the user. We're also passing one argument to read_line: &mut guess. This argument tells
         * it what string to store the user input in.
         * & indicates that this argument is a reference, which gives you a way to let multiple part of
         * your code access one piece of data without needing to copy that data into memory multiple
         * times. Note that, since we have to change the value of guess here, we have to pass it as
         * mutable reference (&mut guess) rather than as an immutable reference (&guess).
         */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Handling porential failure with Result.
        // .real_line() returns a value of type io::Result, which is an enumeration(enum) that can be
        // either Ok or Err.
        // Ok variant indicates the operation was successful, and inside Ok is the successfully
        // generated value.
        // Err variant means the operation failed, and Err contains information about how or why the
        // operation failed.
        // Values of the Result type have methods defined on them. For instance, an instance of Result
        // has an expect method. If this instance of Result is an Err value, expect will cause the
        // program to crash and display the message that you passed as an argument to expect. If the
        // read_line method returns an Err, it would likely be the result of an error comming from the
        // underlying OS. If this instance of Result is an Ok value, expect will take the return value
        // that Ok is holding and return just that value.
        // If you don't call expect, the program will compile but with warning.

        /* Printing values with println! placeholders
         * {} is a placeholder for a variable value.
         * If we print the value of a single variable inside a single placeholder, the variable name
         * can go inside the curly brackets. Otherwise (when printing an expression), place empty curly
         * brackets in the format string, the follow the format string with a comma-separated list of
         * expressions to print in each empty curlt bracket placeholder in the same order.
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
