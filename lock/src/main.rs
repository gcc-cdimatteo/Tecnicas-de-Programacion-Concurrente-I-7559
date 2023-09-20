// Realizar un pequeño idle game
// Algunas entidades "extraerán" (generarán aleatoriamente) oro
// Algunas entidades podrán convertir oro en recursos (a gusto)
// Otras entidades podrán convertir combinaciones de recursos en + oro
// Otras entidades podrán solamente consumir recursos
// Periódicamente se reporta por pantalla el nivel de recursos y oro
use std::{
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
};

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
}
fn convert_gold(gold: Arc<RwLock<Gold>>, resource: Arc<RwLock<Resource>>) {}
fn convert_resource(resource: Arc<RwLock<Resource>>, gold: Arc<RwLock<Gold>>) {}
fn consume_resource(resource: Arc<RwLock<Resource>>) {}

fn run_extractions(gold: Arc<RwLock<Gold>>) {
    let mut extractions_handle = vec![];

    for _i in 0..3 {
        let gold_copy = gold.clone();
        let handle = thread::spawn(move || extract_gold(gold_copy));
        extractions_handle.push(handle);
    }

    for handle in extractions_handle {
        handle.join().unwrap()
    }
}

fn run_convert_gold(gold: Arc<RwLock<Gold>>, resource: Arc<RwLock<Resource>>) {
    let mut conversor_handle = vec![];

    for _i in 0..3 {
        let gold_copy = gold.clone();
        let resource_copy = resource.clone();
        let handle = thread::spawn(move || convert_gold(gold_copy, resource_copy));
        conversor_handle.push(handle);
    }

    for handle in conversor_handle {
        handle.join().unwrap()
    }
}

fn run_convert_resource(resource: Arc<RwLock<Resource>>, gold: Arc<RwLock<Gold>>) {
    let mut conversor_handle = vec![];

    for _i in 0..3 {
        let gold_copy = gold.clone();
        let resource_copy = resource.clone();
        let handle = thread::spawn(move || convert_resource(resource_copy, gold_copy));
        conversor_handle.push(handle);
    }

    for handle in conversor_handle {
        handle.join().unwrap()
    }
}

fn run_consume_resource(resource: Arc<RwLock<Resource>>) {
    let mut consumer_handle = vec![];

    for _i in 0..3 {
        let resource_copy = resource.clone();
        let handle = thread::spawn(move || consume_resource(resource_copy));
        consumer_handle.push(handle);
    }

    for handle in consumer_handle {
        handle.join().unwrap()
    }
}

fn print_gold(gold: Arc<RwLock<Gold>>) {}

fn print_resource(resource: Arc<RwLock<Resource>>) {}

fn main() {
    let gold = Arc::new(RwLock::new(Gold { amount: 0 }));
    let resource = Arc::new(RwLock::new(Resource { amount: 0 }));

    let mut process_handle = vec![];

    let g_c = gold.clone();
    let t_re = thread::spawn(move || run_extractions(g_c));
    process_handle.push(t_re);

    let g_c = gold.clone();
    let r_c = resource.clone();
    let t_cg = thread::spawn(move || run_convert_gold(g_c, r_c));
    process_handle.push(t_cg);

    let g_c = gold.clone();
    let r_c = resource.clone();
    let t_cr = thread::spawn(move || run_convert_resource(r_c, g_c));
    process_handle.push(t_cr);

    let r_c = resource.clone();
    let t_cor = thread::spawn(move || run_consume_resource(r_c));
    process_handle.push(t_cor);

    let g_c = gold.clone();
    let t_pg = thread::spawn(move || print_gold(g_c));
    process_handle.push(t_pg);

    let r_c = resource.clone();
    let t_pr = thread::spawn(move || print_resource(r_c));
    process_handle.push(t_pr);

    for h in process_handle {
        h.join().unwrap()
    }
}
