mod variables_mutability;
mod data_types;
mod functions;
use variables_mutability::var_mut;
use data_types::dtypes;
use functions::funk;


fn main() {
    println!("------------------------------------------------------\n");
    println!("| Rust Book Chapter 3 - Common Programming Concepts. |\n");
    println!("------------------------------------------------------\n");
    var_mut();
    dtypes();
    funk();
}
