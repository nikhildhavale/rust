// Enums and Pattern Matching Example
// Rust enums are much more powerful than in most languages!

// 1. SIMPLE ENUM - Like traditional enums
#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time_to_change(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 55,
        }
    }

    fn can_go(&self) -> bool {
        match self {
            TrafficLight::Green => true,
            TrafficLight::Red | TrafficLight::Yellow => false,
        }
    }
}

// 2. ENUM WITH DATA - Each variant can hold different data!
#[derive(Debug)]
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Named fields (like a struct)
    Write(String),              // Single value
    ChangeColor(u8, u8, u8),   // Tuple values (RGB)
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("🚪 Quitting application..."),
            Message::Move { x, y } => println!("📍 Moving to position ({}, {})", x, y),
            Message::Write(text) => println!("✍️  Writing: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("🎨 Changing color to RGB({}, {}, {})", r, g, b)
            }
        }
    }
}

// 3. OPTION ENUM - Built-in, replaces null/nil
// enum Option<T> {
//     Some(T),
//     None,
// }

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else if id == 2 {
        Some(String::from("Bob"))
    } else {
        None
    }
}

// 4. RESULT ENUM - Built-in, for error handling
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn parse_age(input: &str) -> Result<u32, String> {
    match input.parse::<u32>() {
        Ok(age) if age <= 120 => Ok(age),
        Ok(_) => Err("Age seems unrealistic".to_string()),
        Err(_) => Err("Not a valid number".to_string()),
    }
}

// 5. COMPLEX ENUM - Representing different types of data
#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn handle_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("📄 Page loaded"),
        WebEvent::PageUnload => println!("📄 Page unloaded"),
        WebEvent::KeyPress(c) => println!("⌨️  Key pressed: '{}'", c),
        WebEvent::Paste(s) => println!("📋 Pasted: \"{}\"", s),
        WebEvent::Click { x, y } => println!("🖱️  Clicked at ({}, {})", x, y),
    }
}

// 6. ENUM FOR STATE MACHINE
#[derive(Debug, PartialEq)]
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { session_id: String },
    Error { code: u32, message: String },
}

impl ConnectionState {
    fn is_connected(&self) -> bool {
        matches!(self, ConnectionState::Connected { .. })
    }

    fn describe(&self) -> String {
        match self {
            ConnectionState::Disconnected => "Not connected".to_string(),
            ConnectionState::Connecting => "Connecting...".to_string(),
            ConnectionState::Connected { session_id } => {
                format!("Connected (session: {})", session_id)
            }
            ConnectionState::Error { code, message } => {
                format!("Error {}: {}", code, message)
            }
        }
    }
}

// 7. NESTED ENUMS
#[derive(Debug)]
enum Shape {
    Circle(f64),                              // radius
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),                 // three sides
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

fn main() {
    println!("=== Enums and Pattern Matching ===\n");

    // 1. Simple Enum
    println!("--- Traffic Light ---");
    let light = TrafficLight::Red;
    println!("Light: {:?}", light);
    println!("Can go? {}", light.can_go());
    println!("Time to change: {} seconds\n", light.time_to_change());

    // 2. Enum with Data
    println!("--- Messages ---");
    let messages = vec![
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];

    for msg in messages {
        msg.process();
    }
    println!();

    // 3. Option Enum
    println!("--- Option Example ---");
    let result1 = divide(10, 2);
    let result2 = divide(10, 0);

    match result1 {
        Some(value) => println!("10 / 2 = {}", value),
        None => println!("Cannot divide by zero"),
    }

    match result2 {
        Some(value) => println!("10 / 0 = {}", value),
        None => println!("Cannot divide by zero"),
    }

    // Option methods (functional style)
    let user = find_user(1);
    println!("User 1: {}", user.unwrap_or(String::from("Not found")));

    let no_user = find_user(99);
    println!("User 99: {}\n", no_user.unwrap_or(String::from("Not found")));

    // 4. if let - Concise pattern matching
    println!("--- if let Pattern ---");
    if let Some(name) = find_user(2) {
        println!("Found user: {}", name);
    } else {
        println!("User not found");
    }
    println!();

    // 5. Result Enum
    println!("--- Result Example ---");
    let ages = vec!["25", "150", "abc", "30"];

    for age_str in ages {
        match parse_age(age_str) {
            Ok(age) => println!("✅ Valid age: {}", age),
            Err(e) => println!("❌ Error parsing '{}': {}", age_str, e),
        }
    }
    println!();

    // 6. Web Events
    println!("--- Web Events ---");
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('x'),
        WebEvent::Paste(String::from("Hello")),
        WebEvent::Click { x: 100, y: 200 },
    ];

    for event in events {
        handle_event(event);
    }
    println!();

    // 7. Connection State
    println!("--- Connection State Machine ---");
    let states = vec![
        ConnectionState::Disconnected,
        ConnectionState::Connecting,
        ConnectionState::Connected {
            session_id: String::from("abc123"),
        },
        ConnectionState::Error {
            code: 404,
            message: String::from("Not found"),
        },
    ];

    for state in &states {
        println!("{} - Connected: {}", state.describe(), state.is_connected());
    }
    println!();

    // 8. Shapes
    println!("--- Shapes ---");
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for shape in shapes {
        println!("{:?} - Area: {:.2}", shape, shape.area());
    }
    println!();

    // 9. Advanced Pattern Matching
    println!("--- Advanced Patterns ---");
    let number = Some(7);

    match number {
        Some(x) if x < 5 => println!("Less than 5: {}", x),
        Some(x) if x == 7 => println!("Lucky number 7!"),
        Some(x) => println!("Some other number: {}", x),
        None => println!("No number"),
    }

    // 10. Match with ranges
    let temperature = 25;
    match temperature {
        i32::MIN..=0 => println!("🥶 Freezing!"),
        1..=15 => println!("🧊 Cold"),
        16..=25 => println!("😊 Comfortable"),
        26..=35 => println!("🔥 Hot"),
        36..=i32::MAX => println!("🌋 Extremely hot!"),
    }

    println!("\n🎉 You've mastered enums and pattern matching!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light() {
        let red = TrafficLight::Red;
        assert_eq!(red.can_go(), false);
        assert_eq!(red.time_to_change(), 60);

        let green = TrafficLight::Green;
        assert_eq!(green.can_go(), true);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(10, 0), None);
    }

    #[test]
    fn test_parse_age() {
        assert_eq!(parse_age("25"), Ok(25));
        assert!(parse_age("150").is_err());
        assert!(parse_age("abc").is_err());
    }

    #[test]
    fn test_connection_state() {
        let connected = ConnectionState::Connected {
            session_id: String::from("test"),
        };
        assert!(connected.is_connected());

        let disconnected = ConnectionState::Disconnected;
        assert!(!disconnected.is_connected());
    }

    #[test]
    fn test_shape_area() {
        let circle = Shape::Circle(1.0);
        assert!((circle.area() - std::f64::consts::PI).abs() < 0.0001);

        let rect = Shape::Rectangle { width: 4.0, height: 5.0 };
        assert_eq!(rect.area(), 20.0);
    }
}
