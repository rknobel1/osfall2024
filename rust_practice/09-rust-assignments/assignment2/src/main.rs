fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        true
    } 
    else {
        false
    }
}

fn main() {
    let numbers: [i32; 10] = [2, 43, 4, 7, 9, 13, 55, 60, 17, 27];

    for i in 0..numbers.len() {
        let num = numbers[i];
        if is_even(num) {
            println!("{} is even", num);
        }
        else {
            println!("{} is odd", num);
        }

        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        }
        else if num % 3 == 0 {
            println!("Fizz");
        }
        else if num % 5 == 0 {
            println!("Buzz");
        }
    }

    let mut counter = 0;
    let mut sum = 0;
    while counter < numbers.len() {
        sum += numbers[counter];
        counter += 1;
    }

    println!("Sum: {}", sum);

    let mut largest = numbers[0];

    for idx in 1..numbers.len() {
        if numbers[idx] > largest {
            largest = numbers[idx];
        }
    }

    println!("Largest: {}", largest)
}