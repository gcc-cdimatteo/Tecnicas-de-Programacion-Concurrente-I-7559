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
const RANGE_END: i32 = 10;

struct Gold {
    amount: RwLock<i64>,
}

struct Resource {
    amount: RwLock<i64>,
}

fn extract_gold(gold: Arc<Gold>) {
    // let start;

    // while end-start < 15: extrae oro;

    // let end;
    println!("EXTRACT GOLD");
}

fn convert_gold(gold: Arc<Gold>, resource: Arc<Resource>) {
    println!("CONVERT GOLD");
}

fn convert_resource(resource: Arc<Resource>, gold: Arc<Gold>) {
    println!("CONVERT RESOURCE");
}

fn consume_resource(resource: Arc<Resource>) {
    println!("CONSUME RESOURCE");
}

fn run_extract_gold(gold: Arc<Gold>) {
    let mut handler = vec![];
    for _ in RANGE_START..RANGE_END {
        let gold_copy = gold.clone();
        let handle = thread::spawn(move || extract_gold(gold_copy));
        handler.push(handle)
    }
    for h in handler {
        h.join().unwrap()
    }
}

fn run_convert_gold(gold: Arc<Gold>, resource: Arc<Resource>) {
    let mut handler = vec![];
    for _ in RANGE_START..RANGE_END {
        let gold_copy = gold.clone();
        let resource_copy = resource.clone();
        let handle = thread::spawn(move || convert_gold(gold_copy, resource_copy));
        handler.push(handle)
    }
    for h in handler {
        h.join().unwrap()
    }
}

fn run_convert_resource(resource: Arc<Resource>, gold: Arc<Gold>) {
    let mut handler = vec![];
    for _ in RANGE_START..RANGE_END {
        let gold_copy = gold.clone();
        let resource_copy = resource.clone();
        let handle = thread::spawn(move || convert_resource(resource_copy, gold_copy));
        handler.push(handle)
    }
    for h in handler {
        h.join().unwrap()
    }
}

fn run_consume_resource(resource: Arc<Resource>) {
    let mut handler = vec![];
    for _ in RANGE_START..RANGE_END {
        let resource_copy = resource.clone();
        let handle = thread::spawn(move || consume_resource(resource_copy));
        handler.push(handle);
    }
    for h in handler {
        h.join().unwrap()
    }
}

fn print_gold(gold: Arc<Gold>) {
    while true {
        println!("PRINT GOLD: {}", gold.amount.read().unwrap());
        sleep(time::Duration::from_secs(2));
    }
}

fn print_resource(resource: Arc<Resource>) {
    while true {
        println!("PRINT RESOURCE: {}", resource.amount.read().unwrap());
        sleep(time::Duration::from_secs(2));
    }
}

fn main() {
    let gold = Arc::new(Gold {
        amount: RwLock::new(0),
    });
    let resource = Arc::new(Resource {
        amount: RwLock::new(0),
    });

    let mut handler = vec![];

    let gold_clone = gold.clone();
    let t = thread::spawn(move || run_extract_gold(gold_clone));
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
