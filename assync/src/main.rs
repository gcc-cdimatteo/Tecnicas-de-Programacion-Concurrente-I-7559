use std::time::{SystemTime, UNIX_EPOCH};

mod con;
mod seq;

fn main() {
    let con_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    con::main();
    let con_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Concurrent time of execution {:?}", con_end - con_start);

    let seq_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    seq::main();
    let seq_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Sequence time of execution {:?}", seq_end - seq_start);

    println!(
        "Concurrent process is {:?}% faster :)",
        ((seq_end - seq_start).as_millis() / (con_end - con_start).as_millis()) * 100
    )
}
