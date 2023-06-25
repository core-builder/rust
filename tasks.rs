fn calculate_sum(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in numbers {
        sum += num;
    }
    sum
}

fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..(number / 2 + 1) {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn find_max_element(array: &[i32]) -> Option<i32> {
    if array.is_empty() {
        return None;
    }
    let mut max = array[0];
    for &num in array {
        if num > max {
            max = num;
        }
    }
    Some(max)
}

fn reverse_string(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

fn is_palindrome(input: &str) -> bool {
    let reversed: String = input.chars().rev().collect();
    input == reversed
}

fn main() {
    // Task 1: Calculate the Sum of Numbers
    let numbers = [1, 2, 3, 4, 5];
    let sum = calculate_sum(&numbers);
    println!("Sum: {}", sum);

    // Task 2: Check if a Number is Prime
    let number = 17;
    let is_prime_number = is_prime(number);
    println!("Is Prime? {}", is_prime_number);

    // Task 3: Find the Maximum Element in an Array
    let array = [5, 8, 3, 1, 9, 2];
    let max_element = find_max_element(&array);
    match max_element {
        Some(max) => println!("Max Element: {}", max),
        None => println!("Array is empty!"),
    }

    // Task 4: Reverse a String
    let input_string = "Hello, Rust!";
   
