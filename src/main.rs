// Note: 
// - At any given time, you can have either one mutable reference or 
// any number of immutable references
// - references must always be valid

fn main() {
    // Ownership and Data
    let a = "hello"; // stored on stack
    let b = String::from("hello"); // stored on heap

    let x = 5;
    let y = x;

    // length, capacity, ptr stored in stack
    // ptr: points to memory where value is held
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); value borrowed here after move

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    // Ownership and Functions
    let s = String::from("hello");
    takes_ownership(s); // s's value is no longer valid

    let x = 5;
    makes_copy(5); // okay to still use x afterwards

    let s5 = gives_ownership();
    let s6 = String::from("hello");
    let s7 = takes_ownership(s6);

    // References and Borrowing
    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The length of '{}' is {}", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);

    let r1 = &s9;
    let r2 = &s9;
    println!("{} and {}", r1, r2);
    let r3 = &mut s9; // r1 and r2 needs to used before

    // Dangling References
    let reference_to_nothing = no_dangle();

    // Slices
    let mut j = String::from("hello world");
    let word = first_word(&j);
    println!("The first word in {} is {}", j, word);

    let c = [1, 2, 3, 4, 5];
    let slice = &c[1..3];
    assert_eq!(slice, &[2, 3]);
} 
// s6 was moved, so nothing happened
// this scope is now over, and variables is no longer valid

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope, memory is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // it does not have ownership of what it refers to, nothing happens

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } s goes out of scope, and is dropped. Memory gone. 

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> &str { // returns slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}