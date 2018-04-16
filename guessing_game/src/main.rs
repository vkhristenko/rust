use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read a line");

    println!("You guessed: {}", guess);

    let x = 10;
    let y = 15;
    println!("x = {} and y = {}", x, y);
}
