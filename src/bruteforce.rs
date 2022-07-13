use ::hex;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128,
};
use std::collections::HashMap;
use std::fs;

const N: usize = 4;

// on cherche collisions avec des N-bytes shake 128 digest identiques
pub fn bruteforce(target: usize) {
    println!("Collisions :\n");
    // hash stuff
    let mut hasher: Shake128;
    let mut reader: sha3::digest::core_api::XofReaderCoreWrapper<sha3::Shake128ReaderCore>;

    // dictionnary stuff
    let mut dict: HashMap<[u8; N], u64> = HashMap::new();

    // vars
    let mut state = 0usize;
    let mut input = 0u64;

    while state < target {
        hasher = Shake128::default();
        hasher.update(&u64::to_be_bytes(input));
        reader = hasher.finalize_xof();
        let mut res = [0u8; N];
        reader.read(&mut res);
        match dict.get(&res) {
            Some(&old_input) => {
                let filenamea = format!("res_bruteforce/collision-{}/file_{}_A", N, state);
                fs::write(filenamea, &u64::to_be_bytes(old_input)).expect("Unable to write file");
                let filenameb = format!("res_bruteforce/collision-{}/file_{}_B", N, state);
                fs::write(filenameb, &u64::to_be_bytes(input)).expect("Unable to write file");
                println!(
                    "H({}) = H({}) = {}",
                    hex::encode(&u64::to_be_bytes(old_input)),
                    hex::encode(&u64::to_be_bytes(input)),
                    hex::encode(&res)
                );
                state += 1;
            }
            None => {
                dict.insert(res, input);
            }
        }
        input += 1;
    }
}
