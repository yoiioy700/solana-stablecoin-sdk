// CHAPTER 3: Structs, Enums, and Pattern Matching
// Custom data types that power Solana programs

fn main() {
    println!("=== CHAPTER 3: STRUCTS, ENUMS, PATTERN MATCHING ===\n");
    
    // =========================================================================
    // PART 1: STRUCTS
    // =========================================================================
    println!("1. STRUCTS (Custom Data Types)\n");
    
    // Classic struct with named fields
    let user1 = User {
        username: String::from("ariq123"),
        email: String::from("ariq@example.com"),
        sign_in_count: 1,
        active: true,
    };
    
    println!("User 1: {} ({}) - Active: {}", 
        user1.username, user1.email, user1.active);
    
    // Using struct update syntax
    let user2 = User {
        email: String::from("ariq2@example.com"),
        username: String::from("ariq456"),
        ..user1  // Copy remaining fields from user1
    };
    println!("User 2: {} - Sign in count: {}\n", 
        user2.username, user2.sign_in_count);
    
    // Tuple struct (named tuple)
    struct Point(i32, i32, i32);
    let origin = Point(0, 0, 0);
    let p = Point(10, 20, 30);
    println!("Point p: ({}, {}, {})", p.0, p.1, p.2);
    println!("Origin: ({}, {}, {})\n", origin.0, origin.1, origin.2);
    
    // Unit struct (no fields, used for markers/traits)
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
    println!("Unit struct created (marker type)\n");
    
    // =========================================================================
    // PART 2: METHODS (impl blocks)
    // =========================================================================
    println!("2. METHODS AND ASSOCIATED FUNCTIONS\n");
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Rectangle: {}x{} = area {}", 
        rect1.width, rect1.height, rect1.area());
    
    let rect2 = Rectangle::square(25);  // Associated function (like static method)
    println!("Square: {}x{} = area {}\n", 
        rect2.width, rect2.height, rect2.area());
    
    // =========================================================================
    // PART 3: ENUMS (Enumerations)
    // =========================================================================
    println!("3. ENUMS (Variants with Data)\n");
    
    // Using Message enum defined below (at file scope)
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);
    
    process_message(&msg1);
    process_message(&msg2);
    process_message(&msg3);
    process_message(&msg4);
    println!();
    
    // =========================================================================
    // PART 4: OPTION AND RESULT ENUMS
    // =========================================================================
    println!("4. OPTION<T> AND RESULT<T, E> (Null Safety)\n");
    
    // Option<T>: Some(value) or None
    let some_number = Some(5);
    let some_string = Some(String::from("hello"));
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Absent: {:?}", absent_number);
    
    // Unwrap safely with match
    match some_number {
        Some(n) => println!("Got number: {}", n),
        None => println!("No number"),
    }
    
    // Result<T, E>: Ok(value) or Err(error)
    let good_result: Result<i32, String> = Ok(42);
    let bad_result: Result<i32, String> = Err(String::from("Something went wrong"));
    
    match &good_result {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Error: {}", e),
    }
    
    // Shorthand with if let
    if let Ok(n) = &good_result {
        println!("Got: {} (via if let)\n", n);
    }
    
    // =========================================================================
    // PART 5: PATTERN MATCHING (match)
    // =========================================================================
    println!("5. PATTERN MATCHING\n");
    
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter { state: String },  // Variant with data
    }
    
    let coins = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter { state: String::from("Alaska") },
    ];
    
    for coin in &coins {
        let value = match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter { state } => {
                println!("Quarter from {}!", state);
                25
            }
        };
        println!("  -> Value: {} cents", value);
    }
    println!();
    
    // =========================================================================
    // PART 6: MATCH WITH RANGES AND GUARDS
    // =========================================================================
    println!("6. ADVANCED PATTERN MATCHING\n");
    
    let number = 7;
    
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("Prime!"),
        4..=10 => println!("Between 4 and 10"),
        _ => println!("Something else"),
    }
    
    // Match with guards
    let num = Some(17);
    match num {
        Some(n) if n < 10 => println!("Less than 10: {}", n),
        Some(n) if n % 2 == 0 => println!("Even: {}", n),
        Some(n) => println!("Odd >= 10: {}", n),
        None => println!("No number"),
    }
    println!();
    
    // =========================================================================
    // PART 7: DESTRUCTURING
    // =========================================================================
    println!("7. DESTRUCTURING\n");
    
    // Destructure struct
    let User { username, email, .. } = &user1;
    println!("Destructured: {} <{}>", username, email);
    
    // Destructure enum
    if let Message::Move { x, y } = msg2 {
        println!("Move to ({}, {})", x, y);
    }
    
    // Destructure tuple
    let tuple = (1, "hello", 3.14);
    let (a, b, c) = tuple;
    println!("Tuple destructure: {}, {}, {}\n", a, b, c);
    
    // =========================================================================
    // PART 8: SOLANA-STYLE EXAMPLES
    // =========================================================================
    println!("8. SOLANA-STYLE PATTERNS\n");
    
    // Account type (like in Solana programs)
    enum Account {
        Initialized { owner: String, balance: u64 },
        Uninitialized,
        Frozen { reason: String },
    }
    
    let accounts = [
        Account::Initialized { 
            owner: String::from("user1"), 
            balance: 1000 
        },
        Account::Uninitialized,
        Account::Frozen { 
            reason: String::from("suspicious_activity") 
        },
    ];
    
    for account in &accounts {
        match account {
            Account::Initialized { owner, balance } => {
                println!("Account: owner={}, balance={}", owner, balance);
            }
            Account::Uninitialized => {
                println!("Account: uninitialized");
            }
            Account::Frozen { reason } => {
                println!("Account: FROZEN - {}", reason);
            }
        }
    }
    
    println!("\n=== CHAPTER 3 COMPLETE ===");
    println!("\nKEY TAKEAWAYS:");
    println!("- Structs: Custom data types with named fields");
    println!("- Tuple structs: Named tuples");
    println!("- Methods: Associated functions using impl blocks");
    println!("- Enums: Variants with different data types");
    println!("- Option<T>: Express nullability safely (Some/None)");
    println!("- Result<T,E>: Error handling (Ok/Err)");
    println!("- Pattern matching: Exhaustive case handling");
    println!("- Destructuring: Extract data from structs/enums");
}

// =========================================================================
// DEFINITIONS
// =========================================================================

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method (takes self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Associated function (no self, like constructor)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Helper to print message enum
fn process_message(msg: &Message) {
    match msg {
        Message::Quit => println!("  Quit message"),
        Message::Move { x, y } => println!("  Move to ({}, {})", x, y),
        Message::Write(text) => println!("  Write: '{}'", text),
        Message::ChangeColor(r, g, b) => println!("  Color: RGB({}, {}, {})", r, g, b),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
