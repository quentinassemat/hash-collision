// Pollard rho en suivant le pseudo code des slides
use ::hex;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128,
};
use std::collections::HashSet;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

use rand::Rng;

const N: usize = 6;
const TREADS_NUMBER: usize = 50;

pub fn f(input: &[u8; N]) -> [u8; N] {
    let mut hasher = Shake128::default();
    let mut reader: sha3::digest::core_api::XofReaderCoreWrapper<sha3::Shake128ReaderCore>;
    hasher.update(input);
    reader = hasher.finalize_xof();
    let mut output = [0u8; N];
    reader.read(&mut output);
    output
}

// pour fiter x0 dans l'ensemble X
pub fn into_x(x0: [u8; 8], n: usize) -> Vec<u8> {
    if n == 8 {
        return Vec::from(x0);
    } else if N < 8 {
        let mut res: Vec<u8> = Vec::new();
        for i in 0..n {
            res.push(x0[i]);
        }
        return res;
    } else {
        let mut res: Vec<u8> = Vec::new();
        for i in 0..8 {
            res.push(x0[i]);
        }
        for _i in 8..n {
            res.push(0u8);
        }
        return res;
    }
}

pub fn pollard_rho_multithreaded(target: usize) {
    println!("Collisions :\n");

    // Hashset pour surveiller qu'on ne retombe pas toujours sur la même collision
    let seen = Arc::new(Mutex::new(HashSet::new()));

    // vars
    let state = Arc::new(Mutex::new(0usize));
    let mut threads = vec![];
    for i in 0..TREADS_NUMBER {
        let seen = seen.clone();
        let state = Arc::clone(&state);
        threads.push(thread::spawn(move || {
            //random pour les x0
            let mut rng = rand::thread_rng();
            let mut x0: u64 = rng.gen();
            // println!("x0 : {} thread {}", x0, i);
            loop {
                {
                    let guard_state = state.lock().unwrap();
                    if *guard_state >= target {
                        break;
                    }
                }
                let x0_bytes_vec = into_x(u64::to_be_bytes(x0), N);
                let mut x0_bytes = [0u8; N];
                for i in 0..N {
                    x0_bytes[i] = x0_bytes_vec[i];
                }
                let mut t = f(&x0_bytes);
                let mut h = f(&t);
                // println!("Thread {} while 1", i);
                while t != h {
                    t = f(&t);
                    h = f(&f(&h));
                }
                let mut t1 = t;
                let mut t2 = x0_bytes;
                let mut t1_prime = f(&t1);
                let mut t2_prime = f(&t2);
                // println!("Thread {} while 2", i);
                while t1_prime != t2_prime {
                    t1 = t1_prime;
                    t2 = t2_prime;
                    t1_prime = f(&t1);
                    t2_prime = f(&t2);
                }
                {
                    let mut current_seen = seen.lock().unwrap();
                    let mut current_state = state.lock().unwrap();
                    if !(*current_seen).contains(&t1_prime) {
                        let filenamea = format!(
                            "res_pollard_rho_multithreaded/collision-{}/file_{}_A",
                            N, *current_state
                        );
                        fs::write(filenamea, &t1).expect("Unable to write file");
                        let filenameb = format!(
                            "res_pollard_rho_multithreaded/collision-{}/file_{}_B",
                            N, *current_state
                        );
                        fs::write(filenameb, &t2).expect("Unable to write file");
                        println!(
                            "H({}) = H({}) = {} (thread {})",
                            hex::encode(&t1),
                            hex::encode(&t2),
                            hex::encode(&t1_prime),
                            i
                        );
                        (*current_seen).insert(t1_prime);
                        *current_state += 1;
                    }
                }
                x0 = rng.gen();
            }
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
}
