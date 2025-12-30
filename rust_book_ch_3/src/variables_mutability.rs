pub fn var_mut() {
    println!("| Variables and Mutability |\n");
    // Variables and mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadow value of x
    // Curly brackets create an inner scope
    {
        // Shadowing the value of the last decalred variable
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x remains the last value in the scope: {x}");

    // Declaring constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}\n")
}