fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess < secret {
        -1
    }
    else if guess > secret {
        1
    }
    else {
        0
    }
}

fn main() {
    // Commented out to remove warning
    // let mut secret = 12;
    let secret = 12;
    let mut guesses = 0;

    let guess_array: [i32; 5] = [7, 10, 17, 15, 12];
    let mut counter = 0;
    let mut guess;

    loop {
        // Commented out to remove warning. Use mut for user input
        guess = guess_array[counter];
        guesses += 1;

        let result = check_guess(guess, secret);
        if result == -1 {
            println!("Guess was too low!");
        }
        else if result == 1 {
            println!("Guess was too high!");
        }
        else {
            println!("Correct! Number of guesses: {}", guesses);
            break;
        }

        counter += 1;
    }
}