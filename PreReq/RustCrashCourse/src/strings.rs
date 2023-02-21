pub fn run() {
    //Immutable fixed length
    // let hello = "hello";
    // println!("{}", hello);

    // Growable heap allocated data structure
    let mut hello = String::from("Hello ");
    println!("{}", hello);
    println!("Length: {}", hello.len());

    //push a char
    hello.push('W');

    // push a string and not just a char
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());
    println!("Is Empty: {}", hello.is_empty());

    //Contains
    println!("Contains 'World' {}", hello.contains("World"));

    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
