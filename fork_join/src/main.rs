use std::time::{SystemTime, UNIX_EPOCH};

mod con;
mod seq;

fn main() {
    let paths: [&str; 10] = [
        "CAvideos", "DEvideos", "FRvideos", "GBvideos", "INvideos", "JPvideos", "KRvideos",
        "MXvideos", "RUvideos", "USvideos",
    ];

    let con_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let con_data = con::process(paths);
    let con_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Concurrent time of execution {:?}", con_end - con_start);

    let seq_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seq_data = seq::process(paths);
    let seq_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Sequence time of execution {:?}", seq_end - seq_start);

    let mut equal = true;

    for (k, v) in con_data.iter() {
        if !seq_data.contains_key(k) {
            equal = false;
            break;
        }
        let seq_val = seq_data.get_key_value(k).unwrap().1;
        if seq_val != v {
            equal = false;
            break;
        }
    }

    println!("{:?}", equal);
}
