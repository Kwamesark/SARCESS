// Defining the is_even function
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Creating an array of 10 integers
    let numbers = [12, 15, 7, 10, 3, 6, 9, 20, 5, 30];

    // for loop is use to iterate through the array
    for &num in numbers.iter() {
        // Printing the Outcome of the number whether  is even or odd
        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }

        // Checking if the number is divisibil by 3 and/or 5
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }

    // Using a while loop to find and print the sum of all numbers
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Using a loop to find and print the largest number
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is: {}", largest);
}