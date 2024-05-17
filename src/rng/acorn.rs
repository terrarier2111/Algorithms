use super::{finalize, xor_shift::{self, gen_u64}, Rng};

pub type AcornRng = GenericAcornRng<16>;

pub struct GenericAcornRng<const STATE_SIZE: usize> {
    seed: [u64; STATE_SIZE],
}

impl<const STATE_SIZE: usize> GenericAcornRng<STATE_SIZE> {

    // FIXME: is this a decent power of two?
    const M: u64 = 1 << 42;

    pub fn new() -> Self {
        Self {
            seed: {
                let mut gen = xor_shift::XorShiftPRng64::DEFAULT_SEED;
                let mut seed = [0; STATE_SIZE];
                let mut i = 0;
                while i < STATE_SIZE {
                    let (new_gen, mut val) = gen_u64(gen);
                    if val % 2 == 0 {
                        val += 1;
                    }
                    gen = new_gen;
                    seed[i] = val % Self::M;
                    assert!(seed[i] % 2 == 1);
                    i += 1;
                }
                seed
            },
        }
    }

    pub fn gen_u64(&mut self) -> u64 {
        for i in 1..STATE_SIZE {
            self.seed[i] = (self.seed[i - 1] + self.seed[i]) % Self::M;
        }
        self.seed[STATE_SIZE - 1]
    }
    
}

impl Rng for AcornRng {
    fn new() -> Self {
        Self::new()
    }

    fn with_seed(_seed: u64) -> Self {
        unimplemented!()
    }

    fn gen_u64(&mut self) -> u64 {
        self.gen_u64()
    }
}