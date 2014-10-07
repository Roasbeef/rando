use std::rand::{Rng, SeedableRng};
use std::num;

static LCG_MULTIPLIER: u32 = 1103515245;
static LCG_INCREMENT: u32 = 12345;
static LCG_MODULOUS: u32 = 2147483647; // 2 ** 31 - 1
static LCG_DEFAULT_SEED: u32 = 1;

struct LinearCongruentialPRNG {
    seed: u32,
    rand_state: u32
}

impl LinearCongruentialPRNG {
    fn new_unseeded() -> LinearCongruentialPRNG {
        LinearCongruentialPRNG { seed: LCG_DEFAULT_SEED, rand_state: LCG_DEFAULT_SEED }
    }
}

impl Rng for LinearCongruentialPRNG {
    fn next_u32(&mut self) -> u32 {
        let next_rand = ((self.rand_state * LCG_MULTIPLIER) + LCG_INCREMENT) & LCG_MODULOUS;
        self.rand_state = next_rand;
        next_rand
    }
}

impl SeedableRng<u32> for LinearCongruentialPRNG {
    fn reseed(&mut self, seed: u32) {
        self.seed = 
            if seed == 0 {
                1
            } else { 
                seed
            }
    }

    fn from_seed(seed: u32) -> LinearCongruentialPRNG {
        LinearCongruentialPRNG { seed: seed, rand_state: seed }
    }
}

enum PRNG {
    LinearCongruentialPRNG,
}

fn main() {
    let mut rng = LinearCongruentialPRNG::new_unseeded();
    for _ in range(0u, 20) {
        println!("Rand num {}", rng.next_u32() % 100);
    }
}

#[test]
fn LCG_test() {
}
