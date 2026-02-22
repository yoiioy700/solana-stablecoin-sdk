// CHAPTER 2: OWNERSHIP - Rust's Most Important Concept
// Ownership = Memory safety without garbage collector

fn main() {
    println!("=== CHAPTER 2: OWNERSHIP ===\n");
    
    // =========================================================================
    // RULES OF OWNERSHIP:
    // 1. Each value has ONE owner
    // 2. When owner goes out of scope, value is dropped
    // 3. There can only be ONE owner at a time
    // =========================================================================
    
    // =====================================
    // PART 1: MOVE SEMANTICS (Transfer)
    // =====================================
    println!("1. MOVE SEMANTICS (String, Vec, etc.)\n");
    
    let s1 = String::from("hello");
    println!("s1 = {}", s1);
    
    let s2 = s1; // s1's value MOVED to s2
    // println!("s1 = {}", s1); // ERROR! s1 no longer valid
    println!("s2 = {} (s1 has been moved)\n", s2);
    
    // =====================================
    // PART 2: CLONE (Deep copy)
    // =====================================
    println!("2. CLONE (Expensive deep copy)\n");
    
    let s3 = String::from("world");
    let s4 = s3.clone(); // Heap data copied
    println!("s3 = {}", s3);
    println!("s4 = {} (cloned from s3)\n", s4);
    
    // =====================================
    // PART 3: COPY TRAIT (Stack-only types)
    // =====================================
    println!("3. COPY TRAIT (Stack types: i32, bool, char, etc.)\n");
    
    let x = 5;
    let y = x; // x is COPIED (not moved) because i32 implements Copy
    println!("x = {} (still valid!)", x);
    println!("y = {} (copied from x)\n", y);
    
    // Types with Copy: all integers, floats, bool, char, tuples of Copy types
    
    // =====================================
    // PART 4: OWNERSHIP AND FUNCTIONS
    // =====================================
    println!("4. OWNERSHIP IN FUNCTIONS\n");
    
    let s5 = String::from("function test");
    takes_ownership(s5); // s5's value moved into function
    // println!("s5 = {}", s5); // ERROR! s5 no longer valid
    println!("(s5 moved to function and dropped)\n");
    
    let x2 = 5;
    makes_copy(x2); // x2 is Copy, still valid
    println!("x2 = {} (still valid after function call)\n", x2);
    
    // =====================================
    // PART 5: RETURN VALUES AND OWNERSHIP
    // =====================================
    println!("5. RETURN VALUES TRANSFER OWNERSHIP\n");
    
    let s6 = gives_ownership(); // Return value moved to s6
    println!("s6 = {}", s6);
    
    let s7 = takes_and_gives_back(s6); // s6 moved in, return moved to s7
    // println!("s6 = {}", s6); // ERROR!
    println!("s7 = {} (s6 moved, new String returned)\n", s7);
    
    // =====================================
    // PART 6: BORROWING (References)
    // =====================================
    println!("6. BORROWING (References - and T)\n");
    
    let s8 = String::from("borrow me");
    let len = calculate_length(&s8); // Pass reference, s8 still owns
    println!("'{}' length = {}", s8, len);
    println!("s8 still valid = {}\n", s8);
    
    // =====================================
    // PART 7: MUTABLE REFERENCES
    // =====================================
    println!("7. MUTABLE REFERENCES (and mut T)\n");
    
    let mut s9 = String::from("hello");
    change_string(&mut s9);
    println!("Changed string: {}\n", s9);
    
    // RULE: You can have only ONE mutable reference at a time
    let r1 = &mut s9;
    // let r2 = &mut s9; // ERROR! Can not borrow mutable twice
    println!("r1 = {} (only one mutable borrow allowed)\n", r1);
    
    // But you can have multiple immutable references
    let s10 = String::from("immutable");
    let ref1 = &s10;
    let ref2 = &s10;
    println!("ref1 = {}", ref1);
    println!("ref2 = {} (multiple immutable borrows OK)\n", ref2);
    
    // =====================================
    // PART 8: DANGLING REFERENCES
    // =====================================
    println!("8. NO DANGLING REFERENCES (Compile-time prevention)\n");
    
    // This will NOT compile:
    // fn dangle() -> &String {
    //     let s = String::from("dangle");
    //     &s // ERROR! s will be dropped, can't return reference
    // }
    
    // This WILL compile (return ownership instead):
    let s11 = no_dangle();
    println!("s11 = {} (returned ownership, not reference)\n", s11);
    
    // =====================================
    // PART 9: SLICES
    // =====================================
    println!("9. SLICES (and [Type] and str)\n");
    
    let s12 = String::from("hello world");
    let hello = &s12[0..5]; // Slice: reference to part
    let world = &s12[6..11];
    println!("full: {}", s12);
    println!("first word: {}", hello);
    println!("second word: {}\n", world);
    
    // String literal IS a slice
    let literal: &str = "immutable string slice";
    println!("String literal (str): {}\n", literal);
    
    // Array slice
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("Array slice [1..4]: {:?}\n", slice);
    
    println!("=== CHAPTER 2 COMPLETE ===");
    println!("\nKEY TAKEAWAYS:");
    println!("- Ownership prevents double-free and use-after-free");
    println!("- Move semantics for heap data (String, Vec)");
    println!("- Copy for stack data (i32, bool)");
    println!("- Borrow using references - temporary access");
    println!("- Mutable borrows are exclusive (1 at a time)");
    println!("- Immutable borrows can be shared (many at a time)");
    println!("- Slices are references to contiguous sequences");
}

// =====================================
// HELPER FUNCTIONS
// =====================================

fn takes_ownership(some_string: String) {
    println!("Received ownership of: {}", some_string);
} // some_string dropped here

fn makes_copy(some_integer: i32) {
    println!("Copied integer: {}", some_integer);
} // some_integer is Copy, nothing special

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // Return moves ownership to caller
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("Received: {}", a_string);
    a_string // Return moves ownership back
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but we don't have ownership, so nothing dropped

fn change_string(s: &mut String) {
    s.push_str(", world!");
}

fn no_dangle() -> String {
    let s = String::from("no dangle");
    s // Return ownership, not reference
}
