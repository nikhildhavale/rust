// Welcome to Rust! This is your first Rust program.
// Let's explore some fundamental concepts.

fn main() {
    // 1. Variables and Mutability
    println!("=== Variables and Mutability ===");
    let x = 5; // Immutable by default
    println!("The value of x is: {}", x);

    let mut y = 10; // Mutable variable
    println!("The value of y is: {}", y);
    y = 15;
    println!("The new value of y is: {}", y);

    // 2. Data Types
    println!("\n=== Data Types ===");
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = '🦀'; // Rust mascot Ferris!
    println!("Integer: {}, Float: {}, Boolean: {}, Character: {}",
             integer, float, boolean, character);

    // 3. Strings
    println!("\n=== Strings ===");
    let string_slice: &str = "Hello"; // String slice (immutable)
    let string_object: String = String::from("World"); // Owned string
    println!("{}, {}!", string_slice, string_object);

    // 4. Arrays and Vectors
    println!("\n=== Arrays and Vectors ===");
    let array: [i32; 3] = [1, 2, 3]; // Fixed size
    let mut vector: Vec<i32> = vec![4, 5, 6]; // Dynamic size
    vector.push(7);
    println!("Array: {:?}", array);
    println!("Vector: {:?}", vector);

    // 5. Functions
    println!("\n=== Functions ===");
    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);

    // 6. Control Flow
    println!("\n=== Control Flow ===");
    let number = 7;
    if number < 5 {
        println!("{} is less than 5", number);
    } else if number < 10 {
        println!("{} is between 5 and 10", number);
    } else {
        println!("{} is 10 or greater", number);
    }

    // 7. Loops
    println!("\n=== Loops ===");
    print!("Counting: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // 8. Pattern Matching
    println!("\n=== Pattern Matching ===");
    let result = divide(10, 2);
    match result {
        Some(value) => println!("10 / 2 = {}", value),
        None => println!("Cannot divide by zero!"),
    }

    // 9. Ownership (fundamental Rust concept)
    println!("\n=== Ownership Example ===");
    let s1 = String::from("Rust");
    let s2 = s1.clone(); // Clone to avoid move
    println!("s1: {}, s2: {}", s1, s2);

    // 10. Borrowing
    println!("\n=== Borrowing Example ===");
    let message = String::from("Learning Rust is fun!");
    print_length(&message); // Borrow instead of move
    println!("Still have access to message: {}", message);

    println!("\n🎉 You've completed your first Rust program!");
}

/// Adds two numbers together
fn add(a: i32, b: i32) -> i32 {
    a + b // Last expression is returned (no semicolon)
}

/// Safely divides two numbers, returning None if divisor is zero
fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

/// Prints the length of a string slice (borrowed)
fn print_length(s: &String) {
    println!("Length of '{}' is {} characters", s, s.len());
}

// Unit tests - run with 'cargo test'
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(7, 0), None);
    }
}
