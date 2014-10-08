use std::rand::{Rng, SeedableRng};

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

struct LinearFeedbackShiftRegisterPRNG {
    front_ptr: *mut u32,
    rear_ptr:  *mut u32,
    end_ptr: *mut u32,
    iv: Vec<u32>
}

impl LinearFeedbackShiftRegisterPRNG {
    fn init_seed_info(seed: &Vec<u32>) -> (*mut u32, *mut u32, *mut u32, Vec<u32>, u32) {
        let mut iv = seed.clone();

        let (deg, shift_sep) = match iv.len() {
            // x**63 + x + 1.  
            x if x >= 64 => (63u, 1u),
            // x**31 + x**3 + 1.
            x if x >= 32 => (31u, 3u),
            // x**15 + x + 1.
            x if x >= 16 => (15u, 1u),
            // x ** 7 + x** 3 + 1
            x if x >= 8 => (7u, 3u),
            _ => (iv.len() - 1, 0),
        };

        for i in range(1u, deg + 1u) {
            let word = (16807 * iv[i-1]) % 2147483647;
            *iv.get_mut(i) = word as u32;
        }

        let front_ptr = unsafe{ iv.as_mut_ptr().offset(deg as int) };
        let rear_ptr = unsafe{ iv.as_mut_ptr().offset(shift_sep as int) };
        let end_ptr = unsafe{ iv.as_mut_ptr().offset((iv.len() - 1) as int)};
        (front_ptr, rear_ptr, end_ptr, iv, deg as u32)
    }
}

impl Rng for LinearFeedbackShiftRegisterPRNG {
    fn next_u32(&mut self) -> u32 {
        let (mut front_ptr, mut rear_ptr, end_ptr) = (self.front_ptr, self.rear_ptr, self.end_ptr);

        let next_rand: u32;
        unsafe {
            *front_ptr += *rear_ptr;
            next_rand = (*front_ptr >> 1) & 0x7fffffff;
        }

        front_ptr = unsafe{ front_ptr.offset(1) };
        if front_ptr >= end_ptr {
            front_ptr = self.iv.as_mut_ptr();
            rear_ptr = unsafe{ rear_ptr.offset(1) };
        } else {
            rear_ptr = unsafe{ rear_ptr.offset(1) };
            if rear_ptr >= end_ptr {
                rear_ptr = self.iv.as_mut_ptr();
            }
        }

        self.front_ptr = front_ptr;
        self.rear_ptr = rear_ptr;

        next_rand
    }
}

impl SeedableRng<Vec<u32>> for LinearFeedbackShiftRegisterPRNG {
    fn reseed(&mut self, seed: Vec<u32>) {
        let reseed_info = LinearFeedbackShiftRegisterPRNG::init_seed_info(&seed);
        self.front_ptr = reseed_info.clone().val0();
        self.rear_ptr = reseed_info.clone().val1();
        self.end_ptr = reseed_info.clone().val2();
        self.iv = reseed_info.clone().val3();

        let deg = reseed_info.clone().val4();

        for _ in range(0, (deg + 1) * 10) {
            let _ = self.next_u32();
        }
    }

    fn from_seed(seed: Vec<u32>) -> LinearFeedbackShiftRegisterPRNG {
        let (front_ptr, rear_ptr, end_ptr, iv, deg) = LinearFeedbackShiftRegisterPRNG::init_seed_info(&seed);

        let mut rng = LinearFeedbackShiftRegisterPRNG{ 
            front_ptr: front_ptr, rear_ptr: rear_ptr, end_ptr: end_ptr, iv: iv
        };

        for _ in range(0, (deg + 1) * 10) {
            rng.next_u32();
        }

        rng
    }
}


enum PRNG {
    LinearCongruentialPRNG,
    LinearFeedbackShiftRegisterPRNG
}

fn main() {
    let mut rng: LinearCongruentialPRNG = SeedableRng::from_seed(5);
    for _ in range(0u, 10) {
        println!("Rand num {}", rng.next_u32() % 100);
    }

    println!("\n");

    rng.reseed(434);
    for _ in range(0u, 10) {
        println!("Rand num {}", rng.next_u32() % 100);
    }

    println!("\n\n");

    let mut seed = vec![3, 434, 545,45, 5454, 6454, 4545, 232424, 52345235, 35434534, 2342341];
    let mut rng: LinearFeedbackShiftRegisterPRNG = SeedableRng::from_seed(seed);
    for _ in range(3u, 20) {
        println!("Rand num {}", rng.next_u32() % 100);
    }
}

#[test]
fn LCG_test() {
}
