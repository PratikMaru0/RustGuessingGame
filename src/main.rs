// Guesing Game
// 1). User se input lenge
// 2). Random Number generate karenge
// 3). inputVal = 10
// user will guess a number if user guessed number is less than inputVal we'll tell them to input greater number and if user guessed number id greater than inputVal we'll tell them to input lesser number. If number guessed is right then print you have guessed number right and you are the winner

use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game");

    let mut secret_number = rand::thread_rng().gen_range(1..101);
    // println!("Number is : {}",secret_Number);  // Dont print this otherwise user will get to know and they will become winner without doing anything HAHAHAHAHHAHHAHAHHAHA............

    let mut count = 0;

    loop {
        println!("Hey User please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line ");

        println!("Your guessed is {}", guess);

        // // Type conversion becoz we have taken user input in string and we are comparing it with the randon number that is generated is integer type. So lets do type casting or type conversion
        let guess: u32 = guess.trim().parse().expect("Type an integer");

        // println!("Number {}" , guess+1 ); // To check whether its converted into integer or not

        // // Now we will do comparision
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You are the winner");
                println!("Chances taken : {}", count + 1);
                break;
            }
        }
        count = count + 1;
    }
}
