//use std::env;

pub mod tile;
pub mod board;

// use crate::tile;
// use crate::board;

fn main() {
    //let mut args = env::args();
    let s = String::from("\n
                          | 2 |   |   |   |   |\n
                          |   |   |   |   |   |\n
                          |   |   |   |   |   |\n
                          |   |   |   |   |   |\n
                          |   |   |   |   |   |\n");

    println!("{}", s);
}
