fn main() {
    let secret = 42; // Hard-coded secret number
    let mut guess = 0; // Initial guess
    let mut num_guesses = 0; // Track the number of guesses

    // Define the check_guess function
    fn check_guess(guess: i32, secret: i32) -> i32 {
        if guess == secret {
            0
        } else if guess > secret {
            1
        } else {
            -1
        }
    }

    // Loop until the correct guess is made
    loop {
        num_guesses += 1; // Increment the number of guesses
        let result = check_guess(guess, secret);

        match result {
            0 => {
                println!("Congratulations! You guessed the correct number {} in {} attempts.", secret, num_guesses);
                break;
            }
            1 => println!("Your guess {} is too high!", guess),
            -1 => println!("Your guess {} is too low!", guess),
            _ => println!("Unexpected result: {}", result), // Catch-all pattern
        }

        // Simulate user input by incrementing the guess
        guess += 1;
    }
}