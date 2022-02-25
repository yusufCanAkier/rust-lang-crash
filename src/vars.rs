pub fn run(){
    let name = "Yusuf";
    let mut age = 21;

    age = 99;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}" , ID);

    // Assign multiple vars

    let (my_name, my_age) = ("Yusuf", 21);
    println!("{} is {}", my_name,my_age);
}