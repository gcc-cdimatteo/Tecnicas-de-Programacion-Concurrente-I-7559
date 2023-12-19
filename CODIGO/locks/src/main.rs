use core::time;
// Realizar un pequeño idle game
// Algunas entidades "extraerán" (generarán aleatoriamente) oro
// Algunas entidades podrán convertir oro en recursos (a gusto)
// Otras entidades podrán convertir combinaciones de recursos en + oro
// Otras entidades podrán solamente consumir recursos
// Periódicamente se reporta por pantalla el nivel de recursos y oro
use std::{
    sync::{Arc, RwLock},
    thread::{self, sleep},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const RANGE_START: i32 = 0;
const RANGE_END: i32 = 10;
const MAX_DURATION: Duration = Duration::new(20, 0);

struct Gold {
    amount: RwLock<i64>,
}

struct Resource {
    amount: RwLock<i64>,
}

fn extract_gold(gold: Arc<Gold>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < MAX_DURATION {
        {
            *gold.amount.write().unwrap() += 2;
        }
        sleep(time::Duration::from_millis(10));
    }
}

fn convert_gold(gold: Arc<Gold>, resource: Arc<Resource>) {
    // 4 Gold --> 8 Resource
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < MAX_DURATION {
        {
            let mut gold_amount = gold.amount.write().unwrap();
            let mut resource_amount = resource.amount.write().unwrap();

            if *gold_amount >= 4 {
                *gold_amount -= 4;
                *resource_amount += 8;
            }
        }
        sleep(time::Duration::from_millis(10));
    }
}

fn convert_resource(resource: Arc<Resource>, gold: Arc<Gold>) {
    // 2 Resources --> 1 Gold
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < MAX_DURATION {
        {
            let mut gold_amount = gold.amount.write().unwrap();
            let mut resource_amount = resource.amount.write().unwrap();

            if *resource_amount >= 2 {
                *resource_amount -= 2;
                *gold_amount += 1;
            }
        }
        sleep(time::Duration::from_millis(10));
    }
}

fn consume_resource(resource: Arc<Resource>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < MAX_DURATION {
        {
            let mut resource_amount = resource.amount.write().unwrap();

            if *resource_amount >= 5 {
                *resource_amount -= 5;
            }
        }
        sleep(time::Duration::from_millis(10));
    }
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
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < 2 * MAX_DURATION {
        {
            println!("PRINT GOLD: {}", *gold.amount.read().unwrap());
        }
        sleep(time::Duration::from_millis(100));
    }
}

fn print_resource(resource: Arc<Resource>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < 2 * MAX_DURATION {
        {
            println!("PRINT RESOURCES: {}", *resource.amount.read().unwrap());
        }
        sleep(time::Duration::from_millis(100));
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
