use super::Rng;

pub struct LCGGenerator64 {
    state: u64,
}

impl LCGGenerator64 {

    const DEFAULT_SEED: u64 = 0xDEADBEEFCAFEBABE;

    pub const fn new() -> Self {
        Self::with_seed(Self::DEFAULT_SEED)
    }

    pub const fn with_seed(seed: u64) -> Self {
        Self {
            state: seed,
        }
    }

    pub fn gen_u64(&mut self) -> u64 {
        // FIXME: use decent constants
        self.state = (0x651965 * self.state + 0x1156781) % 0x621478266278326;
        // FIXME: maybe add some more iterations
        self.state
    }

}

impl Rng for LCGGenerator64 {
    fn new() -> Self {
        Self::new()
    }

    fn with_seed(seed: u64) -> Self {
        Self::with_seed(seed)
    }

    fn gen_u64(&mut self) -> u64 {
        self.gen_u64()
    }
}