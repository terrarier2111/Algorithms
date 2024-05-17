pub mod xor_shift;
pub mod lcg;
pub mod acorn;

pub trait Rng {

    fn new() -> Self;

    fn with_seed(seed: u64) -> Self;

    fn gen_u64(&mut self) -> u64;

}

const fn finalize(val: u64) -> u64 {
    /// This is a constant that's frequently used as a finalizer
    const FINALIZER: u64 = 0xB504F3349AF6D6DC;

    (val ^ (val >> 4) ^ (val << 6)) ^ FINALIZER
}