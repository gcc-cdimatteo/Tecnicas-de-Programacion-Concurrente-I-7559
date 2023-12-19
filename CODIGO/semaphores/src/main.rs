use core::time;
use std::{
    sync::{Arc, RwLock},
    thread,
};
use std_semaphore::Semaphore;

fn main() {
    let bowl_no_vacio = Arc::new(Semaphore::new(0));
    let bowl_no_lleno = Arc::new(Semaphore::new(20));
    let canincas = Arc::new(RwLock::new(0));

    let mut handler: Vec<thread::JoinHandle<_>> = vec![];

    let bowl_no_vacio_fran = bowl_no_vacio.clone();
    let bowl_no_lleno_fran = bowl_no_lleno.clone();
    let canincas_fran = canincas.clone();
    let fran = thread::spawn(move || loop {
        bowl_no_lleno_fran.acquire();

        {
            let mut lock_canica = canincas_fran.write().unwrap();
            *lock_canica += 1;
            println!("Fran tira canica, hay {}", *lock_canica);
        }

        bowl_no_vacio_fran.release();
    });
    handler.push(fran);

    for integrante in [
        "Caro", "Fiore", "Ali", "Fer", "Chloe", "Auri", "Titina", "Meli", "Carli", "Marce", "Mate",
        "Osqui",
    ] {
        let bowl_no_vacio_integrante = bowl_no_vacio.clone();
        let bowl_no_lleno_integrante = bowl_no_lleno.clone();
        let canicas_integrante = canincas.clone();
        let t = thread::spawn(move || loop {
            bowl_no_vacio_integrante.acquire();

            {
                let mut lock_canica = canicas_integrante.write().unwrap();
                *lock_canica -= 1;
                println!("{} agarro canica, quedan {}", integrante, *lock_canica);
            }

            bowl_no_lleno_integrante.release();
            thread::sleep(time::Duration::from_secs(2));
        });
        handler.push(t);
    }

    for handle in handler {
        handle.join().unwrap();
    }

    println!("Marble game finished!");
}
