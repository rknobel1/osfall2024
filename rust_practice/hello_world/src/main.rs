// ASSIGNMENT 1 ------------------------------------------
// // const FAHRENHEIT:f64 = 32.0;

// fn fahrenheit_to_celcius(f: f64) -> f64 {
//     (f - 32.0) * 5.0 / 9.0
// }

// // fn celcius_to_fahrenheit(c: f64) -> f64 {
// //     c * 9.0 / 5.0 + 32.0
// // }

// fn main() {
//     let temp: f64 = 32.0;
//     let mut c_temp = fahrenheit_to_celcius(temp);
//     println!("{}", c_temp);

//     let mut counter = 1.0;
//     while counter <= 5.0 {
//         c_temp = fahrenheit_to_celcius(temp + counter);
//         println!("{}", c_temp);
//         counter += 1.0
//     }
// }

// ASSIGNMENT 2: -----------------------------------
// fn is_even(n: i32) -> bool {
//     if n % 2 == 0 {
//         true
//     } 
//     else {
//         false
//     }
// }

// fn main() {
//     let numbers: [i32; 10] = [2, 43, 4, 7, 9, 13, 55, 60, 17, 27];

//     for i in 0..numbers.len() {
//         let num = numbers[i];
//         if is_even(num) {
//             println!("{} is even", num);
//         }
//         else {
//             println!("{} is odd", num);
//         }

//         if num % 3 == 0 && num % 5 == 0 {
//             println!("FizzBuzz");
//         }
//         else if num % 3 == 0 {
//             println!("Fizz");
//         }
//         else if num % 5 == 0 {
//             println!("Buzz");
//         }
//     }

//     let mut counter = 0;
//     let mut sum = 0;
//     while counter < numbers.len() {
//         sum += numbers[counter];
//         counter += 1;
//     }

//     println!("Sum: {}", sum);

//     let mut largest = numbers[0];

//     for idx in 1..numbers.len() {
//         if numbers[idx] > largest {
//             largest = numbers[idx];
//         }
//     }

//     println!("Largest: {}", largest)
// }

// ASSIGNMENT 3 ------------------------------
// fn check_guess(guess: i32, secret: i32) -> i32 {
//     if guess < secret {
//         -1
//     }
//     else if guess > secret {
//         1
//     }
//     else {
//         0
//     }
// }

// fn main() {
//     let secret = 12;
//     let mut guesses = 0;

//     loop {
//         let mut guess = 15;
//         guesses += 1;

//         let result = check_guess(guess, secret);
//         if result == -1 {
//             println!("Guess was too low!");
//         }
//         else if result == 1 {
//             println!("Guess was too high!");
//         }
//         else {
//             println!("Correct! Number of guesses: {}", guesses);
//             break;
//         }

    
//         break;
//     }
// }