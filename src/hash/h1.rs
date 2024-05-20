use std::hash::Hasher;

pub struct H1 {
    val: u64,
}

impl H1 {

    pub const fn new() -> Self {
        Self {
            val: MIX,
        }
    }

}

const MIX: u64 = 0b01010101011001010100101010101100111100101000100101011000101001011;

const PADDING: u64 = 0b1100101010100010010010101011010110110110111100100101001010100101;

impl Hasher for H1 {
    fn finish(&self) -> u64 {
        self.val
    }

    fn write(&mut self, bytes: &[u8]) {
        for part in bytes.chunks(8) {
            if part.len() == 8 {
                let val = {
                    let mut val = 0;
                    for x in 0..8 {
                        val |= (part[x] as u64) << (x * 8);
                    }
                    val
                };
                let off = (val % 62) + 1;
                if off % 2 == 0 {
                    self.val ^= self.val << off;
                } else {
                    self.val ^= self.val >> off;
                }
                self.val ^= (val >> 3) ^ (val << 5) ^ (val << 29) ^ (val >> 23);
            } else {
                for byte in part {
                    // ensure zeros aren't making our resulting hash 0
                    let val = (*byte % 62) + 1;
                    if val % 2 == 0 {
                        self.val ^= self.val << val;
                    } else {
                        self.val ^= self.val >> val;
                    }
                }
            }
        }
    }
}