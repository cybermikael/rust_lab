mod variables_mutability;
mod data_types;
mod functions;
mod comments;
mod control_flow;
use variables_mutability::var_mut;
use data_types::dtypes;
use functions::funk;
use comments::comms;
use control_flow::control_flow;


fn main() {
    println!("------------------------------------------------------\n");
    println!("| Rust Book Chapter 3 - Common Programming Concepts. |\n");
    println!("------------------------------------------------------\n");
    var_mut();
    dtypes();
    funk();
    comms();
    control_flow();
}
