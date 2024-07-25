pub fn run() {
    let hello = "Hello";
    let mut hello_mut = String::from("Hello ");
    println!("{}", hello);
    println!("{}", hello_mut);
    // Get length
    println!("Length: {}", hello.len());
    println!("Length: {}", hello_mut.len());
    // Push char
    hello_mut.push('W');
    // Push string
    hello_mut.push_str("orld!");
    println!("{}", hello_mut);
    // Capacity in bytes
    println!("Capacity: {}", hello_mut.capacity());
    // Check if empty
    println!("Is Empty: {}", hello_mut.is_empty());
    // Contains
    println!("Contains 'World' {}", hello_mut.contains("World"));
    // Replace
    println!("Replace: {}", hello_mut.replace("World", "There"));
    // Loop through string by whitespace
    for word in hello_mut.split_whitespace() {
        println!("{}", word);
    }
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);
    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);
}