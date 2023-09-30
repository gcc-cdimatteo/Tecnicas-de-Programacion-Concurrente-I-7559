use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{thread, time};

const MAX_DURATION: Duration = Duration::new(10, 0);

#[derive(Debug, Clone)]
struct CafeteraFrancesa {
    hay_cafe: Arc<(Mutex<bool>, Condvar)>,
}

impl CafeteraFrancesa {
    pub fn new() -> Self {
        CafeteraFrancesa {
            hay_cafe: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    pub fn hacer_cafe(&mut self) {
        let (lock, cvar) = &*self.hay_cafe;
        // wait while the condition is true (= there is still coffee in the coffee maker)
        let mut guard = cvar
            .wait_while(lock.lock().unwrap(), |hay_cafe| *hay_cafe)
            .unwrap();

        // making marvellous Folgers Coffee (French Vanilla)
        println!("Fran making coffee...");
        thread::sleep(time::Duration::from_secs(2));
        *guard = true;

        // Notify family members that again there is coffee
        cvar.notify_all();
    }

    pub fn servir_cafe(&mut self, family_member: &str) {
        let (lock, cvar) = &*self.hay_cafe;
        // wait while the condition is true (= there is no coffee in the coffee maker)
        let mut guard = cvar
            .wait_while(lock.lock().unwrap(), |hay_cafe| !*hay_cafe)
            .unwrap();

        // Serving incredible Franch Vainilla Coffee

        if family_member == "Chloe" {
            println!("{} 🐕‍🦺 serving coffee, guau guau guau guau", family_member);
        } else {
            println!("{} serving coffee, miam miam", family_member);
        }
        thread::sleep(time::Duration::from_secs(1));
        *guard = false;

        // Notify family members that there is no more coffee
        cvar.notify_all();
    }
}

fn main() {
    let cafetera = CafeteraFrancesa::new();

    let mut handler: Vec<thread::JoinHandle<_>> = vec![];

    let coffee_time = Arc::new(Mutex::new(true));

    let barrier = Arc::new(Barrier::new(5));

    let mut cafetera_fran = cafetera.clone();
    let coffee_time_fran = coffee_time.clone();
    let fran = thread::spawn(move || {
        while *coffee_time_fran.lock().unwrap() {
            cafetera_fran.hacer_cafe();
        }

        println!("Fran terminó de hacer café");
    });
    handler.push(fran);

    for i in ["Caro", "Fiore", "Fer", "Ali", "Chloe"] {
        let mut cafetera_family = cafetera.clone();
        let coffee_time_family = coffee_time.clone();
        let barrier_clone = barrier.clone();
        let integrante_familiar: thread::JoinHandle<_> = thread::spawn(move || {
            let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start < MAX_DURATION {
                cafetera_family.servir_cafe(i);
                // Fairness
                thread::sleep(time::Duration::from_secs(2));
            }

            let ultimo = barrier_clone.wait();
            if ultimo.is_leader() {
                // What happens if this is commented ?

                // Last one drinks two coffees to free Fran
                cafetera_family.servir_cafe(i);

                {
                    *coffee_time_family.lock().unwrap() = false;
                }
            }
        });
        handler.push(integrante_familiar);
    }

    for handle in handler {
        handle.join().unwrap();
    }

    println!("Finish coffee time");
}
