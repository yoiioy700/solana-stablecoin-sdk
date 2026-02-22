// CHAPTER 4: Collections and Error Handling
// Storing multiple values and handling failures gracefully

use std::collections::HashMap;

fn main() {
    println!("=== CHAPTER 4: COLLECTIONS & ERROR HANDLING ===\n");
    
    // =========================================================================
    // PART 1: VECTOR (Vec<T>)
    // =========================================================================
    println!("1. VECTOR (Vec<T>) - Dynamic array\n");
    
    // Create empty vector
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(10);
    v1.push(15);
    println!("v1 after pushes: {:?}", v1);
    
    // Create with vec! macro
    let v2 = vec![1, 2, 3, 4, 5];
    println!("v2 with vec! macro: {:?}", v2);
    
    // Reading elements - indexing (panics if out of bounds)
    let third = &v2[2];
    println!("Third element (indexing): {}", third);
    
    // Safe reading with get() - returns Option<&T>
    match v2.get(2) {
        Some(val) => println!("Third element (safe): {}", val),
        None => println!("No third element"),
    }
    
    let out_of_bounds = v2.get(100);
    println!("Out of bounds: {:?} (returns None)\n", out_of_bounds);
    
    // Iterating
    println!("Iterating over v2:");
    for i in &v2 {
        println!("  {}", i);
    }
    
    // Mutable iteration
    println!("\nDoubling v1:");
    for i in &mut v1 {
        *i *= 2;  // Dereference to modify
        println!("  {}", i);
    }
    println!();
    
    // =========================================================================
    // PART 2: STRING (String vs &str)
    // =========================================================================
    println!("2. STRING (Growable, mutable, UTF-8)\n");
    
    // Creating strings
    let s1 = String::from("hello");
    let s2 = "world".to_string();  // Same thing
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    
    // Appending
    let mut s3 = String::from("foo");
    s3.push_str("bar");  // Append string slice
    s3.push('!');        // Append single char
    println!("s3 after push: {}", s3);
    
    // Concatenation
    let s4 = s1 + " " + &s2;  // s1 is MOVED here!
    // println!("s1: {}", s1);  // ERROR! s1 moved
    println!("s4 (concatenated): {}", s4);
    
    // Format! macro (doesn't take ownership)
    let s5 = format!("{}-{}-{}!", s2, s2, s2);
    println!("s5 with format!: {}\n", s5);
    
    // Slicing strings (byte indices!)
    let hello = String::from("Здравствуйте");  // Cyrillic, 2 bytes per char
    let slice = &hello[0..4];  // First 2 chars (4 bytes)
    println!("Cyrillic slice [0..4]: {}", slice);
    
    // Iterating over strings
    println!("Chars in 'hello':");
    for c in "hello".chars() {
        print!("{} ", c);
    }
    println!("\n");
    
    // =========================================================================
    // PART 3: HASHMAP
    // =========================================================================
    println!("3. HASHMAP (Key-Value storage)\n");
    
    // Creating
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    
    println!("Scores: {:?}", scores);
    
    // Accessing - returns Option<&V>
    let alice_score = scores.get("Alice");
    println!("Alice's score: {:?}", alice_score);
    
    // Or with match
    match scores.get("Dave") {
        Some(score) => println!("Dave: {}", score),
        None => println!("No score for Dave"),
    }
    
    // Update or insert (entry API)
    scores.entry("Dave").or_insert(0);  // Insert if not exists
    println!("After entry for Dave: {:?}", scores);
    
    // Update based on old value
    let bob_score = scores.entry("Bob").or_insert(0);
    *bob_score += 5;  // Add 5 bonus points
    println!("Bob's updated score: {:?}\n", scores.get("Bob"));
    
    // Iterating
    println!("All scores:");
    for (name, score) in &scores {
        println!("  {}: {}", name, score);
    }
    println!();
    
    // =========================================================================
    // PART 4: ERROR HANDLING
    // =========================================================================
    println!("4. ERROR HANDLING - Result<T, E>\n");
    
    // Recoverable errors with Result
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Cannot divide by zero!"))
        } else {
            Ok(a / b)
        }
    }
    
    // Pattern matching on Result
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error (expected): {}\n", e),
    }
    
    // Using expect (panics on error - use for prototyping)
    // let result = divide(10.0, 2.0).expect("Division failed");
    
    // Using unwrap (panics on error - avoid in production)
    // let result = divide(10.0, 2.0).unwrap();
    
    // Propagating errors with ?
    fn calculate_average(values: &[f64]) -> Result<f64, String> {
        if values.is_empty() {
            return Err(String::from("Empty array"));
        }
        
        let sum: f64 = values.iter().sum();
        let count = values.len() as f64;
        divide(sum, count)  // ? propagates error if Err
    }
    
    let numbers = vec![10.0, 20.0, 30.0];
    match calculate_average(&numbers) {
        Ok(avg) => println!("Average: {}", avg),
        Err(e) => println!("Error: {}", e),
    }
    
    match calculate_average(&[]) {
        Ok(avg) => println!("Average: {}", avg),
        Err(e) => println!("Error (expected): {}\n", e),
    }
    
    // =========================================================================
    // PART 5: COMBINING RESULT WITH OPTIONS
    // =========================================================================
    println!("5. COMBINING RESULT AND OPTION\n");
    
    // Chaining operations
    fn get_first_as_string(v: &[i32]) -> Option<String> {
        v.first().map(|n| n.to_string())
    }
    
    let numbers = vec![42, 7, 13];
    match get_first_as_string(&numbers) {
        Some(s) => println!("First as string: {}", s),
        None => println!("No elements"),
    }
    
    // and_then / or_else
    let result: Result<i32, &str> = Ok(5);
    let doubled = result.map(|n| n * 2);
    println!("Mapped Result: {:?}", doubled);
    
    // =========================================================================
    // PART 6: SOLANA-STYLE EXAMPLES
    // =========================================================================
    println!("\n6. SOLANA-STYLE PATTERNS\n");
    
    // Token balances (like in Solana)
    let mut token_balances: HashMap<String, u64> = HashMap::new();
    token_balances.insert(String::from("USDC"), 1_000_000);  // 1 USDC (6 decimals)
    token_balances.insert(String::from("SOL"), 5_000);      // 0.005 SOL (9 decimals)
    token_balances.insert(String::from("BONK"), 100_000_000);
    
    println!("Token balances:");
    for (token, balance) in &token_balances {
        println!("  {}: {}", token, balance);
    }
    
    // Transfer function (returning Result)
    fn transfer(
        balances: &mut HashMap<String, u64>,
        token: &str,
        amount: u64,
    ) -> Result<(), String> {
        let balance = balances.get_mut(token)
            .ok_or_else(|| format!("Token {} not found", token))?;
        
        if *balance < amount {
            return Err(format!(
                "Insufficient balance: have {}, need {}",
                balance, amount
            ));
        }
        
        *balance -= amount;
        println!("Transferred {} {}", amount, token);
        Ok(())
    }
    
    // Attempt transfers
    println!("\nAttempting transfers:");
    match transfer(&mut token_balances, "USDC", 500_000) {
        Ok(()) => println!("Success!"),
        Err(e) => println!("Failed: {}", e),
    }
    
    match transfer(&mut token_balances, "USDC", 999_999_999) {
        Ok(()) => println!("Success!"),
        Err(e) => println!("Failed (expected): {}", e),
    }
    
    match transfer(&mut token_balances, "NONEXISTENT", 100) {
        Ok(()) => println!("Success!"),
        Err(e) => println!("Failed (expected): {}", e),
    }
    
    println!("\nUpdated balances:");
    for (token, balance) in &token_balances {
        println!("  {}: {}", token, balance);
    }
    
    // Account list (Vec)
    println!("\n7. ACCOUNT MANAGEMENT\n");
    
    #[derive(Debug)]
    struct Account {
        address: String,
        balance: u64,
        is_active: bool,
    }
    
    let mut accounts: Vec<Account> = vec![
        Account {
            address: String::from("7x9f...a2B3"),
            balance: 1_000_000,
            is_active: true,
        },
        Account {
            address: String::from("8y0g...b4C5"),
            balance: 500_000,
            is_active: false,
        },
    ];
    
    // Add new account
    accounts.push(Account {
        address: String::from("9z1h...c6D7"),
        balance: 2_000_000,
        is_active: true,
    });
    
    // Filter and process
    println!("Active accounts:");
    for account in accounts.iter().filter(|a| a.is_active) {
        println!("  {:?}", account);
    }
    
    // Find specific account
    let target = "7x9f...a2B3";
    match accounts.iter().find(|a| a.address == target) {
        Some(account) => println!("\nFound account {}: {:?}", target, account),
        None => println!("\nAccount {} not found", target),
    }
    
    println!("\n=== CHAPTER 4 COMPLETE ===");
    println!("\nKEY TAKEAWAYS:");
    println!("- Vec<T>: Dynamic array, growable");
    println!("- String: Growable UTF-8 string (owned)");
    println!("- &str: String slice (borrowed)");
    println!("- HashMap<K, V>: Key-value storage");
    println!("- Result<T, E>: Error handling (Ok/Err)");
    println!("- Option<T>: Nullability (Some/None)");
    println!("- ? operator: Error propagation");
    println!("- unwrap/expect: Use sparingly (can panic)");
}