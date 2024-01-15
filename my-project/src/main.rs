pub mod lib;

fn main() {
    println!("Calling add = {}", lib::add(20, 30));
    println!("Calling minus = {}", lib::minus(20, 30));
    println!("Calling multiple: = {}", lib::multiply(20, 30));
}
