// Structs and Methods Example
// This demonstrates how Rust replaces classes with structs, impl blocks, and traits

use std::fmt;

// 1. BASIC STRUCT - Like a class without methods
#[derive(Debug)] // Automatically implement Debug trait for printing
struct Dog {
    name: String,
    breed: String,
    age: u32,
    is_good_dog: bool, // Spoiler: always true
}

// 2. IMPL BLOCK - Add methods to the struct
impl Dog {
    // Associated function (like a static method or constructor)
    // Note: 'new' is just a convention, not special syntax
    fn new(name: String, breed: String, age: u32) -> Self {
        Dog {
            name,
            breed,
            age,
            is_good_dog: true, // All dogs are good dogs!
        }
    }

    // Method that borrows self immutably (&self)
    fn bark(&self) {
        println!("{} says: Woof woof!", self.name);
    }

    // Method that borrows self immutably and returns a value
    fn describe(&self) -> String {
        format!(
            "{} is a {} year old {}",
            self.name, self.age, self.breed
        )
    }

    // Method that borrows self mutably (&mut self)
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("🎉 Happy birthday {}! Now {} years old!", self.name, self.age);
    }

    // Method that takes ownership (self)
    fn adopt_out(self) -> String {
        format!("{} has found a forever home! 🏠", self.name)
    }
}

// 3. ANOTHER STRUCT EXAMPLE - Bank Account
struct BankAccount {
    account_number: String,
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: String, owner: String) -> Self {
        BankAccount {
            account_number,
            owner,
            balance: 0.0,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("💰 Deposited ${:.2}. New balance: ${:.2}", amount, self.balance);
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        println!("💸 Withdrew ${:.2}. New balance: ${:.2}", amount, self.balance);
        Ok(())
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

// 4. TRAITS - Define shared behavior (like interfaces)
trait Vehicle {
    fn start(&self);
    fn stop(&self);
    fn honk(&self);
}

// 5. MULTIPLE STRUCTS CAN IMPLEMENT THE SAME TRAIT
struct Car {
    brand: String,
    model: String,
    running: bool,
}

impl Car {
    fn new(brand: String, model: String) -> Self {
        Car {
            brand,
            model,
            running: false,
        }
    }
}

impl Vehicle for Car {
    fn start(&self) {
        println!("🚗 {} {} engine started: Vroom vroom!", self.brand, self.model);
    }

    fn stop(&self) {
        println!("🛑 {} {} engine stopped", self.brand, self.model);
    }

    fn honk(&self) {
        println!("🔊 Beep beep!");
    }
}

struct Bicycle {
    brand: String,
    gears: u8,
}

impl Bicycle {
    fn new(brand: String, gears: u8) -> Self {
        Bicycle { brand, gears }
    }
}

impl Vehicle for Bicycle {
    fn start(&self) {
        println!("🚴 Pedaling the {} bicycle!", self.brand);
    }

    fn stop(&self) {
        println!("🛑 Braking the bicycle");
    }

    fn honk(&self) {
        println!("🔔 Ring ring!");
    }
}

// 6. TRAIT WITH DEFAULT IMPLEMENTATION
trait Describable {
    fn description(&self) -> String;

    // Default method implementation
    fn print_description(&self) {
        println!("Description: {}", self.description());
    }
}

impl Describable for Dog {
    fn description(&self) -> String {
        format!("A {} named {}", self.breed, self.name)
    }
}

// 7. STRUCT WITH GENERIC TYPE
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Method only available for Point<f64>
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// MAIN FUNCTION - Demonstrates everything
fn main() {
    println!("=== Structs and Methods in Rust ===\n");

    // 1. Using Dog struct
    println!("--- Dogs Example ---");
    let mut my_dog = Dog::new(
        String::from("Buddy"),
        String::from("Golden Retriever"),
        3,
    );

    println!("{}", my_dog.describe());
    my_dog.bark();
    my_dog.have_birthday();
    println!("Is good dog? {}", my_dog.is_good_dog);
    println!("Debug print: {:?}\n", my_dog);

    // 2. Using BankAccount struct
    println!("--- Bank Account Example ---");
    let mut account = BankAccount::new(
        String::from("123456"),
        String::from("Alice"),
    );

    account.deposit(1000.0);
    account.deposit(500.0);

    match account.withdraw(300.0) {
        Ok(_) => println!("✅ Withdrawal successful"),
        Err(e) => println!("❌ Error: {}", e),
    }

    match account.withdraw(2000.0) {
        Ok(_) => println!("✅ Withdrawal successful"),
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("Final balance: ${:.2}\n", account.get_balance());

    // 3. Using Traits - Polymorphism
    println!("--- Vehicles Example (Traits) ---");
    let car = Car::new(String::from("Toyota"), String::from("Camry"));
    let bike = Bicycle::new(String::from("Trek"), 21);

    // Both implement Vehicle trait
    car.start();
    car.honk();
    car.stop();
    println!();

    bike.start();
    bike.honk();
    bike.stop();
    println!();

    // 4. Function that accepts any Vehicle
    fn test_drive(vehicle: &impl Vehicle) {
        vehicle.start();
        vehicle.honk();
        vehicle.stop();
    }

    println!("--- Test Drive ---");
    test_drive(&car);

    // 5. Trait with default implementation
    println!("\n--- Describable Trait ---");
    let another_dog = Dog::new(
        String::from("Max"),
        String::from("Beagle"),
        5,
    );
    another_dog.print_description();

    // 6. Generic struct
    println!("\n--- Generic Struct Example ---");
    let int_point = Point::new(5, 10);
    let float_point = Point::new(3.0, 4.0);

    println!("Integer point: {:?}", int_point);
    println!("Float point: {:?}", float_point);
    println!("Distance from origin: {:.2}", float_point.distance_from_origin());

    // 7. Ownership example with adopt_out (consumes self)
    println!("\n--- Ownership Example ---");
    let rescue_dog = Dog::new(
        String::from("Charlie"),
        String::from("Mixed"),
        2,
    );
    let message = rescue_dog.adopt_out(); // rescue_dog is moved here
    println!("{}", message);
    // rescue_dog.bark(); // This would error - rescue_dog was moved!

    println!("\n🎉 You've learned structs, methods, and traits!");
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog_creation() {
        let dog = Dog::new(String::from("Test"), String::from("Poodle"), 1);
        assert_eq!(dog.name, "Test");
        assert_eq!(dog.age, 1);
        assert_eq!(dog.is_good_dog, true);
    }

    #[test]
    fn test_dog_birthday() {
        let mut dog = Dog::new(String::from("Test"), String::from("Poodle"), 1);
        dog.have_birthday();
        assert_eq!(dog.age, 2);
    }

    #[test]
    fn test_bank_account_deposit() {
        let mut account = BankAccount::new(
            String::from("123"),
            String::from("Test"),
        );
        account.deposit(100.0);
        assert_eq!(account.get_balance(), 100.0);
    }

    #[test]
    fn test_bank_account_withdraw() {
        let mut account = BankAccount::new(
            String::from("123"),
            String::from("Test"),
        );
        account.deposit(100.0);
        let result = account.withdraw(50.0);
        assert!(result.is_ok());
        assert_eq!(account.get_balance(), 50.0);
    }

    #[test]
    fn test_insufficient_funds() {
        let mut account = BankAccount::new(
            String::from("123"),
            String::from("Test"),
        );
        account.deposit(100.0);
        let result = account.withdraw(200.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_point_distance() {
        let point = Point::new(3.0, 4.0);
        assert_eq!(point.distance_from_origin(), 5.0);
    }
}
