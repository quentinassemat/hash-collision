mod bruteforce;
mod bruteforce_perso;
mod ow;
mod ow_perso;
mod pollard_rho;
mod pollard_rho_multithreaded;

use crate::bruteforce::*;
use crate::bruteforce_perso::*;
use crate::ow::*;
use crate::ow_perso::*;
use crate::pollard_rho::*;
use crate::pollard_rho_multithreaded::*;

const TARGET: usize = 10;
fn main() {
    // bruteforce(TARGET);
    // bruteforce_perso(TARGET);
    // pollard_rho(TARGET);
    // pollard_rho_multithreaded(TARGET);
    // ow(TARGET);
    ow_perso(TARGET);
}
