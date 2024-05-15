pub struct XorShiftPRng64 {
    state: u64,
}

impl XorShiftPRng64 {

    // FIXME: use proper value
    const DEFAULT_SEED: u64 = 0xC3A5B2678A2F3E41;

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
        /// This is a constant that's frequently used as a finalizer
        const FINALIZER: u64 = 0xB504F3349AF6D6DC;

        self.state ^= self.state >> 13;
        self.state ^= self.state << 16;
        self.state ^= self.state >> 23;
        ((self.state >> 4) ^ (self.state << 6)) ^ FINALIZER
    }

}

pub trait Bitsequence {}