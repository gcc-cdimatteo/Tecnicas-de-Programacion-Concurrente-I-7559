use csv::Reader;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    process([
        "CAvideos", "DEvideos", "FRvideos", "GBvideos", "INvideos", "JPvideos", "KRvideos",
        "MXvideos", "RUvideos", "USvideos",
    ]);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", end - start);
}

pub fn process(paths: [&str; 10]) -> HashMap<String, i64> {
    let mut h: HashMap<String, i64> = HashMap::new();

    for path in paths {
        let h2 = process_file(format!("./archive/{}.csv", path));
        hash_merging(&mut h, &h2);
    }

    h
}

fn hash_merging(h1: &mut HashMap<String, i64>, h2: &HashMap<String, i64>) {
    if h2.is_empty() {
        return;
    }

    for (key, val) in h2.iter() {
        add_hashmap_key(h1, key.to_string(), val.to_owned());
    }
}

fn add_hashmap_key(h: &mut HashMap<String, i64>, k: String, v: i64) {
    if h.contains_key(&k) {
        let v_old = h.get(&k).unwrap();
        h.insert(k, v_old + v);
    } else {
        h.insert(k, v);
    }
}

fn process_file(filename: String) -> HashMap<String, i64> {
    let mut rdr = Reader::from_path(filename).unwrap();
    let mut data: HashMap<String, i64> = HashMap::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let channel_title = record.get(3).expect("error").to_string();
        let channel_view = record.get(7).expect("error").parse::<i64>().expect("err");

        add_hashmap_key(&mut data, channel_title, channel_view);
    }

    data
}
