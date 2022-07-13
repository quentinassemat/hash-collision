// Pollard rho en suivant le pseudo code des slides
use ::hex;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128,
};
use std::collections::HashSet;
use std::fs;
use std::thread;

use rand::Rng;

const N: usize = 4;

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
        for i in 8..n {
            res.push(0u8);
        }
        return res;
    }
}

pub fn pollard_rho(target: usize) {
    println!("Collisions :\n");

    // random pour les x0
    let mut rng = rand::thread_rng();

    // vars
    let mut state = 0usize;
    let mut x0: u64 = rng.gen();

    // Hashset pour surveiller qu'on ne retombe pas toujours sur la même collision
    let mut seen = HashSet::new();

    while state < target {
        let x0_bytes_vec = into_x(u64::to_be_bytes(x0), N);
        let mut x0_bytes = [0u8; N];
        for i in 0..N {
            x0_bytes[i] = x0_bytes_vec[i];
        }
        let mut t = f(&x0_bytes);
        let mut h = f(&t);
        while t != h {
            t = f(&t);
            h = f(&f(&h));
        }
        let mut t1 = t;
        let mut t2 = x0_bytes;
        let mut t1_prime = f(&t1);
        let mut t2_prime = f(&t2);
        while t1_prime != t2_prime {
            t1 = t1_prime;
            t2 = t2_prime;
            t1_prime = f(&t1);
            t2_prime = f(&t2);
        }
        if !seen.contains(&t1_prime) {
            let filenamea = format!("res_pollard_rho/collision-{}/file_{}_A", N, state);
            fs::write(filenamea, &t1).expect("Unable to write file");
            let filenameb = format!("res_pollard_rho/collision-{}/file_{}_B", N, state);
            fs::write(filenameb, &t2).expect("Unable to write file");
            println!(
                "H({}) = H({}) = {}",
                hex::encode(&t1),
                hex::encode(&t2),
                hex::encode(&t1_prime)
            );
            state += 1;
            seen.insert(t1_prime);
        }
        x0 = rng.gen();
    }
}
