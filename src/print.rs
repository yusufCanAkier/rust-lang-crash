pub fn run() {
    // print the console
    println!("Hello from the print.rs file!");
    //basic formatting
    println!("{} is from {}", "Yusuf" , "Can");

    // positional arguments 
    println!("{0} is from {1} and {0} likes to {2}", "Yusuf" , "Can" , "code");

    // Named Arguments 
    println!("{name} likes to play {activity}", name="Yusuf", activity="Basketball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal {:o}",10,10,10);

    // Placeholder for debug trait
    println!("{:?}",  (12, true, "say to hi"));

    // basic maths operations
    println!("10 + 10 = {}", 10 + 10);
    println!("{}",10+10);
}

