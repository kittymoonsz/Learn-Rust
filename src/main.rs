    // TESTING OWNERSHIP AND REFERENCES
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.
fn main() {
    //References and Borrowing
    let s1: String = String::from("Helloooooooo");
    let (s2, length) = calculate_length(s1);

    println!("{s2} has length of {length}\n");
    //println!("{s1}"); // This doesn't compile because s1's ownership moved to s2

    let ref1: String = String::from("Hellooooo");
    let len = calculate_length_with_reference(&ref1);

    println!("{ref1} has length of {len}");

    // Slice
    let word = String::from("Hello world");
    let word = first_word(&word);
    println!("{}", word);

}

// References and Borrowing
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}

// Slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
