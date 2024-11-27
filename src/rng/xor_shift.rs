use super::{finalize, Rng};

pub struct XorShiftPRng64 {
    state: u64,
}

impl XorShiftPRng64 {

    // FIXME: use proper value
    pub(crate) const DEFAULT_SEED: u64 = 0xC3A5B2678A2F3E41;

    pub const fn new() -> Self {
        Self::with_seed(Self::DEFAULT_SEED)
    }

    pub const fn with_seed(seed: u64) -> Self {
        // FIXME: scramble up seed
        Self {
            state: seed,
        }
    }

    pub fn rand_u64(&mut self) -> u64 {
        let (state, val) = gen_u64(self.state);
        self.state = state;
        val
    }

}

impl Rng for XorShiftPRng64 {
    fn new() -> Self {
        Self::new()
    }

    fn with_seed(seed: u64) -> Self {
        Self::with_seed(seed)
    }

    fn gen_u64(&mut self) -> u64 {
        self.rand_u64()
    }
}

pub(crate) const fn gen_u64(mut state: u64) -> (u64, u64) {
    state ^= state >> 13;
    state ^= state << 16;
    state ^= state >> 23;
    (state, finalize(state))
}