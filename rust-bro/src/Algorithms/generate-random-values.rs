

// #![feature(rand)]

// extern crate rand;

use rand::Rng;
// use std::io;
// use std::cmp::Ordering;

fn main(){
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random: u8: {}", n1);
    println!("Random: u16: {}", n2);
}