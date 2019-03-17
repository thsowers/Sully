use std::{thread, time};

fn main() {
    let second= time::Duration::from_secs(1);
    let mut sully_seconds = 208;

    println!("Both engines disabled");

    while sully_seconds != 0 {
        println!("{}", sully_seconds);
        sully_seconds -=1 ;
        thread::sleep(second);
    }
    println!("Landed safely");
}
