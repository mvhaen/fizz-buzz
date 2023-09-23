# Agile Technical Practices Distilled: Fizz Buzz kata

> Note: this is just me running through this kata, while simultaneously learning Rust and experimenting with Github Copilot. I used Copilot to both generate large parts of the implementation and tests, and later refactor. Most the documentation below was also generated with Copilot.

The Fizz Buzz kata is a simple programming exercise that is often used to teach or practice basic programming concepts. The goal of the exercise is to write a program that prints the numbers from 1 to 100, but with some modifications:

- If the number is divisible by 3, print "Fizz" instead of the number.
- If the number is divisible by 5, print "Buzz" instead of the number.
- If the number is divisible by both 3 and 5, print "Fizz-Buzz" instead of the number.
  
The exercise is usually done in a group setting, with participants taking turns to write a few lines of code to implement the functionality. The goal is to practice techniques like pair programming, test-driven development, and refactoring, while also getting familiar with the syntax and structure of the programming language being used.

## Installation

GitHub Copilot: Sure! Rust is a systems programming language that is designed to be fast, safe, and concurrent. It is often used for developing high-performance applications, such as operating systems, web servers, and game engines.

To get started with Rust for this repository, you will need to have Rust and Cargo installed on your machine. Here are the steps to install Rust and Cargo:

1. Go to the Rust website at https://www.rust-lang.org/learn/get-started and follow the instructions for your operating system to install Rust.
2. Once Rust is installed, open a terminal or command prompt and type `rustc --version` to verify that Rust is installed correctly. You should see a message like "rustc x.y.z (abcabcabc yyyy-mm-dd)".
3. Next, type `cargo --version` to verify that Cargo is installed. You should see a message like "cargo x.y.z (abcabcabc yyyy-mm-dd)".

That's it! You're now ready to start working with Rust and the Fizz Buzz kata.

## Running the Program

To run the program, follow these steps:

1. Open a terminal or command prompt and navigate to the root directory of the project.
2. Type `cargo run` and press Enter. This will compile the code and run the program.
3. The program will print the numbers from 1 to 100, with "Fizz", "Buzz", or "Fizz-Buzz" substituted for the appropriate multiples of 3 and 5.

## Running the Tests

To run the tests, follow these steps:

1. Open a terminal or command prompt and navigate to the root directory of the project.
2. Type `cargo test` and press Enter. This will compile the code and run all the tests in the project.
3. If all the tests pass, you should see a message like "test result: ok. X passed; 0 failed; 0 ignored; 0 measured; 0 filtered out".

That's it! Running the program and tests using `cargo` is a simple and convenient way to develop Rust applications.

