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

const N: usize = 9;
const TREADS_NUMBER: usize = 8;
const ZEROS: usize = 8;

pub fn from_input_to_mot(input: &[u8; N], mots: &[String; 256]) -> String {
    let mut input_str: String = String::new();
    for i in 0..N {
        input_str.push_str(&mots[input[i] as usize]);
        input_str.push(' ');
    }
    input_str
}

pub fn f(input: &[u8; N], mots: &[String; 256]) -> [u8; N] {
    let mut hasher = Shake128::default();
    let mut reader: sha3::digest::core_api::XofReaderCoreWrapper<sha3::Shake128ReaderCore>;
    let input_str = from_input_to_mot(input, mots);
    hasher.update(input_str.as_bytes());
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

pub fn ow_perso(target: usize) {
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
            // mots
            let mots: [String; 256] = [
                "crayon".to_string(),
                "stylo".to_string(),
                "feutre".to_string(),
                "taille-crayon".to_string(),
                "pointe".to_string(),
                "mine".to_string(),
                "gomme".to_string(),
                "dessin".to_string(),
                "coloriage".to_string(),
                "rayure".to_string(),
                "peinture".to_string(),
                "pinceau".to_string(),
                "couleur".to_string(),
                "craie".to_string(),
                "papier".to_string(),
                "feuille".to_string(),
                "cahier".to_string(),
                "carnet".to_string(),
                "carton".to_string(),
                "ciseaux".to_string(),
                "découpage".to_string(),
                "pliage".to_string(),
                "pli".to_string(),
                "colle".to_string(),
                "affaire".to_string(),
                "boîte".to_string(),
                "casier".to_string(),
                "caisse".to_string(),
                "trousse".to_string(),
                "cartable".to_string(),
                "jouet".to_string(),
                "banane".to_string(),
                "pion".to_string(),
                "dé".to_string(),
                "domino".to_string(),
                "puzzle".to_string(),
                "cube".to_string(),
                "perle".to_string(),
                "chose".to_string(),
                "forme.to_string(),rond".to_string(),
                "pâteàmodeler".to_string(),
                "tampon".to_string(),
                "livre".to_string(),
                "histoire".to_string(),
                "bibliothèque".to_string(),
                "image".to_string(),
                "album".to_string(),
                "titre".to_string(),
                "bandedessinée".to_string(),
                "conte".to_string(),
                "dictionnaire".to_string(),
                "magazine".to_string(),
                "catalogue".to_string(),
                "page".to_string(),
                "ligne".to_string(),
                "mot".to_string(),
                "enveloppe".to_string(),
                "étiquette".to_string(),
                "carte.to_string(),affiche".to_string(),
                "alphabet".to_string(),
                "appareil".to_string(),
                "caméscope".to_string(),
                "cassette".to_string(),
                "cédé".to_string(),
                "cédérom".to_string(),
                "chaîne".to_string(),
                "chanson".to_string(),
                "chiffre".to_string(),
                "contraire".to_string(),
                "différence".to_string(),
                "doigt".to_string(),
                "écran".to_string(),
                "écriture".to_string(),
                "film".to_string(),
                "fois".to_string(),
                "idée".to_string(),
                "instrument".to_string(),
                "intrus".to_string(),
                "lettre".to_string(),
                "liste".to_string(),
                "magnétoscope".to_string(),
                "main".to_string(),
                "micro".to_string(),
                "modèle".to_string(),
                "musique".to_string(),
                "nom".to_string(),
                "nombre".to_string(),
                "orchestre".to_string(),
                "ordinateur".to_string(),
                "photo".to_string(),
                "point".to_string(),
                "poster".to_string(),
                "pouce".to_string(),
                "prénom".to_string(),
                "question".to_string(),
                "radio".to_string(),
                "sens".to_string(),
                "tambour".to_string(),
                "télécommande".to_string(),
                "téléphone".to_string(),
                "télévision".to_string(),
                "trait".to_string(),
                "trompette".to_string(),
                "voix".to_string(),
                "xylophone".to_string(),
                "zéro".to_string(),
                "aiguille".to_string(),
                "ampoule".to_string(),
                "avion".to_string(),
                "bois".to_string(),
                "bout".to_string(),
                "bricolage".to_string(),
                "bruit".to_string(),
                "cabane".to_string(),
                "abricot".to_string(),
                "clou".to_string(),
                "ail".to_string(),
                "crochet".to_string(),
                "élastique".to_string(),
                "ficelle".to_string(),
                "fil".to_string(),
                "marionnette".to_string(),
                "marteau".to_string(),
                "métal".to_string(),
                "mètre".to_string(),
                "morceau".to_string(),
                "moteur".to_string(),
                "objet".to_string(),
                "outil".to_string(),
                "aliment".to_string(),
                "ananas".to_string(),
                "planche".to_string(),
                "plâtre".to_string(),
                "scie".to_string(),
                "tournevis".to_string(),
                "vis".to_string(),
                "voiture".to_string(),
                "véhicule".to_string(),
                "chanter".to_string(),
                "chercher".to_string(),
                "choisir".to_string(),
                "chuchoter".to_string(),
                "coller".to_string(),
                "colorier".to_string(),
                "commencer".to_string(),
                "comparer".to_string(),
                "compter".to_string(),
                "construire".to_string(),
                "continuer".to_string(),
                "copier".to_string(),
                "couper".to_string(),
                "déchirer".to_string(),
                "décoller".to_string(),
                "décorer".to_string(),
                "découper".to_string(),
                "demander".to_string(),
                "démolir".to_string(),
                "sedépêcher".to_string(),
                "dessiner".to_string(),
                "dire".to_string(),
                "discuter".to_string(),
                "écouter".to_string(),
                "écrire".to_string(),
                "effacer".to_string(),
                "entendre".to_string(),
                "entourer".to_string(),
                "envoyer".to_string(),
                "faire".to_string(),
                "finir".to_string(),
                "fouiller".to_string(),
                "goûter".to_string(),
                "imiter".to_string(),
                "laisser".to_string(),
                "lire".to_string(),
                "mettre".to_string(),
                "montrer".to_string(),
                "ouvrir.to_string(),parler".to_string(),
                "peindre".to_string(),
                "plier".to_string(),
                "poser".to_string(),
                "prendre".to_string(),
                "préparer".to_string(),
                "ranger".to_string(),
                "réciter".to_string(),
                "recommencer".to_string(),
                "regarder".to_string(),
                "remettre".to_string(),
                "répéter".to_string(),
                "répondre".to_string(),
                "sentir".to_string(),
                "souligner".to_string(),
                "tailler".to_string(),
                "setaire".to_string(),
                "tenir".to_string(),
                "terminer".to_string(),
                "toucher".to_string(),
                "travailler".to_string(),
                "trier.to_string(),acrobate".to_string(),
                "arrêt".to_string(),
                "arrière".to_string(),
                "barre".to_string(),
                "barreau".to_string(),
                "bord".to_string(),
                "bras".to_string(),
                "cerceau".to_string(),
                "chaises".to_string(),
                "cheville".to_string(),
                "chute".to_string(),
                "cœur".to_string(),
                "corde".to_string(),
                "corps".to_string(),
                "côté".to_string(),
                "cou".to_string(),
                "coude".to_string(),
                "cuisse".to_string(),
                "danger".to_string(),
                "doigts".to_string(),
                "dos".to_string(),
                "échasses".to_string(),
                "échelle".to_string(),
                "épaule".to_string(),
                "équipe".to_string(),
                "escabeau".to_string(),
                "fesse".to_string(),
                "filet".to_string(),
                "fond".to_string(),
                "genou".to_string(),
                "gymnastique".to_string(),
                "hanche".to_string(),
                "jambes".to_string(),
                "jeu".to_string(),
                "mains".to_string(),
                "milieu".to_string(),
                "montagne".to_string(),
                "escalade".to_string(),
                "muscle".to_string(),
                "numéro".to_string(),
                "ongle".to_string(),
                "parcours".to_string(),
                "pas".to_string(),
                "passerelle".to_string(),
                "pente".to_string(),
                "peur".to_string(),
                "pieds".to_string(),
                "plongeoir".to_string(),
                "poignet".to_string(),
                "poing".to_string(),
                "pontdesinge".to_string(),
                "poutred’équilibre".to_string(),
                "prises".to_string(),
                "rivièredescrocodiles".to_string(),
                "roulade".to_string(),
                "épingle".to_string(),
                "bâton".to_string(),
                "bêtise".to_string(),
                "bonhomme".to_string(),
            ];
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
                let mut t = f(&x0_bytes, &mots);
                k += 1;
                // println!("thread {}, while 1", i);
                // println!("après premier lock");
                while !is_distinguished(&t) {
                    t = f(&t, &mots);
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
                                    t1 = f(&t1, &mots);
                                }
                                // println!("t = {:?}, t1 = {:?}", t, t1);
                                // t1 = f^(k - old_k) (x0)
                                let mut t1_prime = f(&t1, &mots);
                                let mut t2_prime = f(&t2, &mots);
                                // println!("thread {}, debut while 2 1", i);
                                while t1_prime != t2_prime {
                                    t1 = t1_prime;
                                    t2 = t2_prime;
                                    // println!("{:?}", t1_prime);
                                    t1_prime = f(&t1, &mots);
                                    t2_prime = f(&t2, &mots);
                                }
                                // println!("thread {}, fin while 2 1", i);
                                {
                                    let mut current_seen = seen.lock().unwrap();
                                    let mut current_state = state.lock().unwrap();
                                    if !(*current_seen).contains(&t1_prime) && (t1 != t2) {
                                        let filenamea = format!(
                                            "res_ow_perso/collision-{}/file_{}_A",
                                            N, *current_state
                                        );
                                        fs::write(filenamea, &from_input_to_mot(&t1, &mots).as_bytes()).expect("Unable to write file");
                                        let filenameb = format!(
                                            "res_ow_perso/collision-{}/file_{}_B",
                                            N, *current_state
                                        );
                                        fs::write(filenameb, &from_input_to_mot(&t2, &mots).as_bytes()).expect("Unable to write file");
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
                                    t2 = f(&t2, &mots);
                                } // t2 = f^(old_k - k) (old_x0)
                                let mut t1_prime = f(&t1, &mots);
                                let mut t2_prime = f(&t2, &mots);
                                // println!("thread {}, debut while 2 2", i);
                                while t1_prime != t2_prime {
                                    t1 = t1_prime;
                                    t2 = t2_prime;
                                    t1_prime = f(&t1, &mots);
                                    t2_prime = f(&t2, &mots);
                                }
                                // println!("thread {}, fin while 2 2", i);
                                {
                                    let mut current_seen = seen.lock().unwrap();
                                    let mut current_state = state.lock().unwrap();
                                    if !(*current_seen).contains(&t1_prime) && (t1 != t2) {
                                        let filenamea = format!(
                                            "res_ow_perso/collision-{}/file_{}_A",
                                            N, *current_state
                                        );
                                        fs::write(filenamea, &from_input_to_mot(&t1, &mots).as_bytes()).expect("Unable to write file");
                                        let filenameb = format!(
                                            "res_ow_perso/collision-{}/file_{}_B",
                                            N, *current_state
                                        );
                                        fs::write(filenameb, &from_input_to_mot(&t2, &mots).as_bytes()).expect("Unable to write file");
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
