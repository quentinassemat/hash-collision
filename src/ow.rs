// van Oorschot and Wiener
use ::hex;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

use rand::Rng;

const N: usize = 4;
const TREADS_NUMBER: usize = 100;
const ZEROS: usize = 3;

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

// à modifier pour faire des tests
// distinguished a
pub fn is_distinguished(x: &[u8; N]) -> bool {
    (x[0] >> (8 - ZEROS)) == 0
}

pub fn ow(target: usize) {
    println!("Collisions :\n");

    // Hashset pour surveiller qu'on ne retombe pas toujours sur la même collision
    let seen = Arc::new(Mutex::new(HashSet::new()));

    // Hashset pour monitor les points distinguables
    let distinguished = Arc::new(Mutex::new(HashMap::new()));

    // vars
    let state = Arc::new(Mutex::new(0usize));
    let mut threads = vec![];
    for i in 0..TREADS_NUMBER {
        let seen = seen.clone();
        let state = state.clone();
        let distinguished = distinguished.clone();
        threads.push(thread::spawn(move || {
            //random pour les x0
            let mut rng = rand::thread_rng();
            let mut x0: u64 = rng.gen();
            let mut k = 0u128;
            // println!("x0 : {} thread {}", x0, i);
            loop {
                // println!("debut loop");
                {
                    let guard_state = state.lock().unwrap();
                    if *guard_state >= target {
                        // println!("break");
                        break;
                    }
                }
                // println!("après premier lock");
                let x0_bytes_vec = into_x(u64::to_be_bytes(x0), N);
                let mut x0_bytes = [0u8; N];
                for i in 0..N {
                    x0_bytes[i] = x0_bytes_vec[i];
                }
                let mut t = f(&x0_bytes);
                k += 1;
                // println!("thread {}, while 1", i);
                // println!("après premier lock");
                while !is_distinguished(&t) {
                    t = f(&t);
                    k += 1; // t  = f^k(x0)
                    if k > (2 << (4 * N)) {
                        // println!("early abort");
                        break;
                    }
                }
                // println!("après premier lock");
                // println!("thread {}, point distingué trouvé", i);
                {
                    let mut guard_distinguished = distinguished.lock().unwrap();
                    match (*guard_distinguished).get(&t) {
                        // S'il y a une collision dans l'ensemble distingué on la cherche en amont
                        Some(&(old_x0_bytes, old_k)) => {
                            // println!("thread {}, remonte la collision", i);
                            // println!(
                            //     "x0 = {:?}, k = {}, old_x0 = {:?}, old_k = {}",
                            //     x0_bytes, k, old_x0_bytes, old_k
                            // );
                            if old_k <= k {
                                let mut t1 = x0_bytes;
                                let mut t2 = old_x0_bytes;
                                for _i in 0..(k - old_k) {
                                    t1 = f(&t1);
                                }
                                // println!("t = {:?}, t1 = {:?}", t, t1);
                                // t1 = f^(k - old_k) (x0)
                                let mut t1_prime = f(&t1);
                                let mut t2_prime = f(&t2);
                                // println!("thread {}, debut while 2 1", i);
                                while t1_prime != t2_prime {
                                    t1 = t1_prime;
                                    t2 = t2_prime;
                                    // println!("{:?}", t1_prime);
                                    t1_prime = f(&t1);
                                    t2_prime = f(&t2);
                                }
                                // println!("thread {}, fin while 2 1", i);
                                {
                                    let mut current_seen = seen.lock().unwrap();
                                    let mut current_state = state.lock().unwrap();
                                    if !(*current_seen).contains(&t1_prime) {
                                        let filenamea = format!(
                                            "res_ow/collision-{}/file_{}_A",
                                            N, *current_state
                                        );
                                        fs::write(filenamea, &t1).expect("Unable to write file");
                                        let filenameb = format!(
                                            "res_ow/collision-{}/file_{}_B",
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
                            } else {
                                let mut t1 = x0_bytes;
                                let mut t2 = old_x0_bytes;
                                for _i in 0..(old_k - k) {
                                    t2 = f(&t2);
                                } // t2 = f^(old_k - k) (old_x0)
                                let mut t1_prime = f(&t1);
                                let mut t2_prime = f(&t2);
                                // println!("thread {}, debut while 2 2", i);
                                while t1_prime != t2_prime {
                                    t1 = t1_prime;
                                    t2 = t2_prime;
                                    t1_prime = f(&t1);
                                    t2_prime = f(&t2);
                                }
                                // println!("thread {}, fin while 2 2", i);
                                {
                                    let mut current_seen = seen.lock().unwrap();
                                    let mut current_state = state.lock().unwrap();
                                    if !(*current_seen).contains(&t1_prime) {
                                        let filenamea = format!(
                                            "res_ow/collision-{}/file_{}_A",
                                            N, *current_state
                                        );
                                        fs::write(filenamea, &t1).expect("Unable to write file");
                                        let filenameb = format!(
                                            "res_ow/collision-{}/file_{}_B",
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
                            }
                        }
                        // Sinon on insert avec les infos nécessaires pour pouvoir remonter plus tard
                        None => {
                            // println!("thread {}, première occurence", i);
                            (*guard_distinguished).insert(t, (x0_bytes, k));
                        }
                    }
                }
                x0 = rng.gen();
                k = 0u128;
            }
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
}
