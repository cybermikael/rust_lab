pub fn funk() {
    println!("| Functions |\n");

    another_function(5, 'h');

    expressions();

    let x = return_values(5);
    println!("The value of x is: {x}")
}

fn another_function(x: i32, unit_label: char) {
    println!("This is another function. The value of x is {x}{unit_label}");
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn return_values(x: i32) -> i32 {
    x + 5
}