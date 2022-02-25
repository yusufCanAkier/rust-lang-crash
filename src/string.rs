pub fn run(){
    let mut hello = String::from("Hello ");
    println!("Length: {}", hello.len());
    hello.push('W');
    hello.push_str("orld!");

    // Capacity in bytes

    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());
    
    // contains
    println!("Contains 'World' {}", hello.contains("World"));

    println!("Replace : {}", hello.replace("World", "Universe"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    println!("{}", hello);
    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('B');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    
    println!("{}", s);
    // Get length



}