use core::time;
// Realizar un pequeño idle game
// Algunas entidades "extraerán" (generarán aleatoriamente) oro
// Algunas entidades podrán convertir oro en recursos (a gusto)
// Otras entidades podrán convertir combinaciones de recursos en + oro
// Otras entidades podrán solamente consumir recursos
// Periódicamente se reporta por pantalla el nivel de recursos y oro
use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const RANGE_START: i32 = 0;
const RANGE_END: i32 = 10;
const MAX_DURATION: Duration = Duration::new(5, 0);

struct Gold {
    amount: Mutex<i64>,
}

struct Resource {
    amount: Mutex<i64>,
}

fn extract_gold(gold: Arc<Gold>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < MAX_DURATION {
        {
            *gold.amount.lock().unwrap() += 2;
        }
    }
}

fn convert_gold(gold: Arc<Gold>, resource: Arc<Resource>) {
    // 4 Gold --> 1 Resource
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < MAX_DURATION {
        {
            let mut gold_amount = *gold.amount.lock().unwrap();

            if gold_amount >= 4 {
                gold_amount -= 4;
                *resource.amount.lock().unwrap() += 1;
            }
        }
    }
}

fn convert_resource(resource: Arc<Resource>, gold: Arc<Gold>) {
    // 2 Resources --> 1 Gold
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < MAX_DURATION {
        {
            let mut resource_amount = *resource.amount.lock().unwrap();

            if resource_amount >= 2 {
                resource_amount -= 2;
                *gold.amount.lock().unwrap() += 1;
            }
        }
    }
}

fn consume_resource(resource: Arc<Resource>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < MAX_DURATION {
        {
            let mut resource_amount = *resource.amount.lock().unwrap();
            if resource_amount >= 1 {
                resource_amount -= 1;
            }
        }
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
        println!("PRINT GOLD: {}", *gold.amount.lock().unwrap());
        sleep(time::Duration::from_secs(1));
    }
}

fn print_resource(resource: Arc<Resource>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < 2 * MAX_DURATION {
        println!("PRINT RESOURCES: {}", *resource.amount.lock().unwrap());
        sleep(time::Duration::from_secs(1));
    }
}

fn main() {
    let gold = Arc::new(Gold {
        amount: Mutex::new(0),
    });
    let resource = Arc::new(Resource {
        amount: Mutex::new(0),
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
