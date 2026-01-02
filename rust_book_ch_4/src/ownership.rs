pub fn ownership() {
    println!("| Ownership |");
    println!("-------------");

    // First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    let mut s = String::from("Mikael");
    println!("My name is: {s}");
    s.push_str(", the best hacker in the universe.");
    println!("{s}");

    // Variables and Data interacting with Move
    let x = 5;
    let y = x;

    println!("{y}");

    // Scope and Assignment
    // The inverse of this is true for the relationship between scoping, ownership, and memory being freed via the drop function as well. When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately. Consider this code, for example:
    let mut s = String::from("Hello");
    s = String::from("Franko");
    println!("{s}, wassup!");

    // Variables and Data Interacting with Clone
    // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone. We’ll discuss method syntax in Chapter 5, but because methods are a common feature in many programming languages, you’ve probably seen them before.
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // Ownership and Functions
    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does. Listing 4-3 has an example with some annotations showing where variables go into and out of scope.

    let s = String::from("Hello");
    take_ownership(s);
    let x = 5;
    makes_copy(x);

    // Return Values and Scope

    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);
}

fn take_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}