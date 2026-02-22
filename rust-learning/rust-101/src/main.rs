// Chapter 1: Variables, Mutability, and Basic Types
// File: src/variables.rs

fn main() {
    println!("=== CHAPTER 1: VARIABLES & BASIC TYPES ===\n");
    
    // 1. Variables are IMMUTABLE by default
    let x = 5;
    println!("Immutable x = {}", x);
    // x = 6; // ERROR! Cannot assign twice to immutable variable
    
    // 2. MUT keyword makes variable mutable
    let mut y = 5;
    println!("Before: y = {}", y);
    y = 10;
    println!("After: y = {}\n", y);
    
    // 3. Constants (type MUST be annotated, UPPER_CASE)
    const MAX_POINTS: u32 = 100_000;
    println!("Constant MAX_POINTS = {}", MAX_POINTS);
    
    // 4. Shadowing (declare new variable with same name)
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("After shadowing: z = {}\n", z);
    
    // 5. Scalar Types
    // Signed integers: i8, i16, i32, i64, i128, isize
    // Unsigned integers: u8, u16, u32, u64, u128, usize
    let a: i32 = -42;
    let b: u64 = 100;
    println!("Signed: {}", a);
    println!("Unsigned: {}", b);
    
    // 6. Floating point
    let c: f64 = 3.14159;
    println!("Float: {}\n", c);
    
    // 7. Boolean
    let is_active: bool = true;
    println!("Boolean: {}", is_active);
    
    // 8. Character (4 bytes, supports Unicode)
    let heart_emoji: char = '😻';
    println!("Character: {}\n", heart_emoji);
    
    // 9. Compound Types: Tuple
    let tup: (i32, f64, &str) = (500, 6.4, "hello");
    let (t1, t2, t3) = tup;
    println!("Tuple destructuring: {}, {}, {}", t1, t2, t3);
    println!("Tuple index 0: {}\n", tup.0);
    
    // 10. Compound Types: Array
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    println!("Array first element: {}", first);
    println!("Array length: {}\n", arr.len());
    
    // 11. String slice (&str) vs String
    let slice: &str = "Hello";  // Immutable string slice
    let mut s = String::from("Hello");  // Growable, mutable String
    s.push_str(", World!");
    println!("String slice: {}", slice);
    println!("String: {}\n", s);
    
    println!("✅ Chapter 1 Complete!");
}
