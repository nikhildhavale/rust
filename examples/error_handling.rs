// Error Handling in Rust
// Learn how to handle errors properly without panicking!

use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;

// 1. BASIC RESULT TYPE
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// 2. CUSTOM ERROR TYPE - Using enum
#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn square_root(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// 3. THE ? OPERATOR - Propagate errors easily
fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // ? returns error if it fails
    let mut username = String::new();
    file.read_to_string(&mut username)?; // ? again
    Ok(username)
}

// Without ? operator (for comparison)
fn read_username_verbose(filename: &str) -> Result<String, io::Error> {
    let file_result = File::open(filename);
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 4. MULTIPLE ERROR TYPES - Converting errors
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(ParseIntError),
    ValidationError(String),
}

// Implement From to enable ? operator with different error types
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}

fn read_age_from_file(filename: &str) -> Result<u32, AppError> {
    let mut file = File::open(filename)?; // io::Error auto-converted to AppError
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let age: u32 = contents.trim().parse()?; // ParseIntError auto-converted

    if age > 150 {
        return Err(AppError::ValidationError(
            String::from("Age seems unrealistic")
        ));
    }

    Ok(age)
}

// 5. VALIDATION FUNCTION
fn validate_email(email: &str) -> Result<(), String> {
    if !email.contains('@') {
        return Err(String::from("Email must contain @"));
    }
    if !email.contains('.') {
        return Err(String::from("Email must contain a domain"));
    }
    if email.len() < 5 {
        return Err(String::from("Email is too short"));
    }
    Ok(())
}

// 6. RECOVERABLE OPERATIONS
struct Database {
    connected: bool,
}

impl Database {
    fn new() -> Self {
        Database { connected: false }
    }

    fn connect(&mut self) -> Result<(), String> {
        // Simulate connection
        self.connected = true;
        Ok(())
    }

    fn query(&self, sql: &str) -> Result<Vec<String>, String> {
        if !self.connected {
            return Err(String::from("Not connected to database"));
        }

        if sql.is_empty() {
            return Err(String::from("Empty query"));
        }

        // Simulate query
        Ok(vec![String::from("result1"), String::from("result2")])
    }

    fn disconnect(&mut self) -> Result<(), String> {
        if !self.connected {
            return Err(String::from("Already disconnected"));
        }
        self.connected = false;
        Ok(())
    }
}

// 7. CHAINING OPERATIONS WITH RESULT
fn process_number(input: &str) -> Result<i32, String> {
    input
        .trim()
        .parse::<i32>()
        .map_err(|e| format!("Parse error: {}", e))?
        .checked_mul(2)
        .ok_or_else(|| String::from("Multiplication overflow"))
}

// 8. USING unwrap_or AND unwrap_or_else
fn get_username(user_id: u32) -> Option<String> {
    if user_id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn main() {
    println!("=== Error Handling in Rust ===\n");

    // 1. Basic Result handling
    println!("--- Basic Result ---");
    match divide(10.0, 2.0) {
        Ok(result) => println!("✅ 10 / 2 = {}", result),
        Err(e) => println!("❌ Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("✅ 10 / 0 = {}", result),
        Err(e) => println!("❌ Error: {}", e),
    }
    println!();

    // 2. Custom Error Types
    println!("--- Custom Error Types ---");
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("✅ Result: {}", result),
        Err(MathError::DivisionByZero) => println!("❌ Cannot divide by zero!"),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    match square_root(-4.0) {
        Ok(result) => println!("✅ Square root: {}", result),
        Err(MathError::NegativeSquareRoot) => {
            println!("❌ Cannot take square root of negative number!")
        }
        Err(e) => println!("❌ Error: {:?}", e),
    }
    println!();

    // 3. The ? operator demonstration
    println!("--- File Reading (will fail gracefully) ---");
    match read_username_from_file("nonexistent.txt") {
        Ok(username) => println!("✅ Username: {}", username),
        Err(e) => println!("❌ Could not read file: {}", e),
    }
    println!();

    // 4. Validation
    println!("--- Email Validation ---");
    let emails = vec![
        "user@example.com",
        "invalid",
        "no-at-sign.com",
        "a@b",
    ];

    for email in emails {
        match validate_email(email) {
            Ok(_) => println!("✅ '{}' is valid", email),
            Err(e) => println!("❌ '{}': {}", email, e),
        }
    }
    println!();

    // 5. Database operations
    println!("--- Database Operations ---");
    let mut db = Database::new();

    // Try query before connecting
    match db.query("SELECT * FROM users") {
        Ok(results) => println!("✅ Results: {:?}", results),
        Err(e) => println!("❌ {}", e),
    }

    // Connect and query
    db.connect().expect("Failed to connect");
    match db.query("SELECT * FROM users") {
        Ok(results) => println!("✅ Results: {:?}", results),
        Err(e) => println!("❌ {}", e),
    }

    db.disconnect().expect("Failed to disconnect");
    println!();

    // 6. Chaining operations
    println!("--- Chaining Operations ---");
    let inputs = vec!["10", "abc", "999999999999999"];

    for input in inputs {
        match process_number(input) {
            Ok(result) => println!("✅ {} * 2 = {}", input, result),
            Err(e) => println!("❌ '{}': {}", input, e),
        }
    }
    println!();

    // 7. unwrap_or and unwrap_or_else
    println!("--- Default Values ---");
    let user1 = get_username(1).unwrap_or(String::from("Guest"));
    let user2 = get_username(99).unwrap_or(String::from("Guest"));

    println!("User 1: {}", user1);
    println!("User 99: {}", user2);

    let user3 = get_username(99).unwrap_or_else(|| {
        println!("Computing default username...");
        String::from("Guest")
    });
    println!("User 99 (with computation): {}", user3);
    println!();

    // 8. Result methods
    println!("--- Result Methods ---");
    let result: Result<i32, &str> = Ok(42);
    println!("Is Ok? {}", result.is_ok());
    println!("Is Err? {}", result.is_err());
    println!("Value or default: {}", result.unwrap_or(0));

    let error_result: Result<i32, &str> = Err("Something went wrong");
    println!("\nError result is Ok? {}", error_result.is_ok());
    println!("Error or default: {}", error_result.unwrap_or(-1));
    println!();

    // 9. Multiple error handling strategies
    println!("--- Error Handling Strategies ---");

    // Strategy 1: Match
    match divide(10.0, 0.0) {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Handled error: {}", e),
    }

    // Strategy 2: if let for simpler cases
    if let Err(e) = divide(5.0, 0.0) {
        println!("Error occurred: {}", e);
    }

    // Strategy 3: unwrap_or for defaults
    let result = divide(10.0, 2.0).unwrap_or(0.0);
    println!("Result with default: {}", result);

    // Strategy 4: map for transforming success values
    let doubled = divide(10.0, 2.0)
        .map(|x| x * 2.0)
        .unwrap_or(0.0);
    println!("Doubled result: {}", doubled);

    // Strategy 5: and_then for chaining operations
    let chained = divide(10.0, 2.0)
        .and_then(|x| divide(x, 2.0))
        .unwrap_or(0.0);
    println!("Chained operations: {}", chained);

    println!("\n🎉 You've mastered error handling in Rust!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert_eq!(safe_divide(10.0, 0.0), Err(MathError::DivisionByZero));
    }

    #[test]
    fn test_square_root() {
        assert_eq!(square_root(4.0), Ok(2.0));
        assert_eq!(square_root(-4.0), Err(MathError::NegativeSquareRoot));
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("no-at-sign.com").is_err());
    }

    #[test]
    fn test_database_connection() {
        let mut db = Database::new();
        assert!(db.query("SELECT *").is_err()); // Not connected

        db.connect().unwrap();
        assert!(db.query("SELECT *").is_ok()); // Connected

        db.disconnect().unwrap();
        assert!(db.query("SELECT *").is_err()); // Disconnected
    }

    #[test]
    fn test_process_number() {
        assert_eq!(process_number("5"), Ok(10));
        assert!(process_number("abc").is_err());
    }
}
