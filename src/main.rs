use crate::garden::vegetables::Asparagus;
use std::{cmp::Ordering, io};

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I am growing {plant:?}!");
}
