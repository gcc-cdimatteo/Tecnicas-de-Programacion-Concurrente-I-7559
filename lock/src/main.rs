use core::time;
// Realizar un pequeño idle game
// Algunas entidades "extraerán" (generarán aleatoriamente) oro
// Algunas entidades podrán convertir oro en recursos (a gusto)
// Otras entidades podrán convertir combinaciones de recursos en + oro
// Otras entidades podrán solamente consumir recursos
// Periódicamente se reporta por pantalla el nivel de recursos y oro
use std::{
    sync::{Arc, RwLock},
    thread::{self, sleep, JoinHandle},
};

const RANGE_START: i32 = 0;
const RANGE_END: i32 = 30;

struct Gold {
    amount: i64,
}

struct Resource {
    amount: i64,
}

fn extract_gold(gold: Arc<RwLock<Gold>>) {
    // let start;

    // while end-start < 15: extrae oro;

    // let end;
    println!("EXTRACT GOLD");
}

fn convert_gold(gold: Arc<RwLock<Gold>>, resource: Arc<RwLock<Resource>>) {
    println!("CONVERT GOLD");
}

fn convert_resource(resource: Arc<RwLock<Resource>>, gold: Arc<RwLock<Gold>>) {
    println!("CONVERT RESOURCE");
}

fn consume_resource(resource: Arc<RwLock<Resource>>) {
    println!("CONSUME RESOURCE");
}

fn run_extractions(gold: Arc<RwLock<Gold>>) {
    for _ in RANGE_START..RANGE_END {
        extract_gold(gold.to_owned());
    }
}

fn run_convert_gold(gold: Arc<RwLock<Gold>>, resource: Arc<RwLock<Resource>>) {
    for _ in RANGE_START..RANGE_END {
        convert_gold(gold.to_owned(), resource.to_owned());
    }
}

fn run_convert_resource(resource: Arc<RwLock<Resource>>, gold: Arc<RwLock<Gold>>) {
    for _ in RANGE_START..RANGE_END {
        convert_resource(resource.to_owned(), gold.to_owned());
    }
}

fn run_consume_resource(resource: Arc<RwLock<Resource>>) {
    for _ in RANGE_START..RANGE_END {
        consume_resource(resource.to_owned());
    }
}

fn print_gold(gold: Arc<RwLock<Gold>>) {
    while true {
        println!("PRINT GOLD");
        sleep(time::Duration::from_secs(2));
    }
}

fn print_resource(resource: Arc<RwLock<Resource>>) {
    while true {
        println!("PRINT RESOURCE");
        sleep(time::Duration::from_secs(2));
    }
}

fn main() {
    let gold = Arc::new(RwLock::new(Gold { amount: 0 }));
    let resource = Arc::new(RwLock::new(Resource { amount: 0 }));

    let mut handler = vec![];

    let gold_clone = gold.clone();
    let t = thread::spawn(move || run_extractions(gold_clone));
    handler.push(t);

    let gold_clone = gold.clone();
    let resource_clone = resource.clone();
    let t = thread::spawn(move || run_convert_gold(gold_clone, resource_clone));
    handler.push(t);

    let gold_clone = gold.clone();
    let resource_clone = resource.clone();
    let t = thread::spawn(move || run_convert_resource(resource_clone, gold_clone));
    handler.push(t);

    let resource_clone = resource.clone();
    let t = thread::spawn(move || run_consume_resource(resource_clone));
    handler.push(t);

    let gold_clone = gold.clone();
    let t = thread::spawn(move || print_gold(gold_clone));
    handler.push(t);

    let resource_clone = resource.clone();
    let t = thread::spawn(move || print_resource(resource_clone));
    handler.push(t);

    for h in handler {
        h.join().unwrap()
    }
}
