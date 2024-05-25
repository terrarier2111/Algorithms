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

    fn write_part(&mut self, val: u64) {
        let val = val ^ MIX;
        let off = (val % 62) + 1;
        if off % 2 == 0 {
            self.val ^= self.val << off;
        } else {
            self.val ^= self.val >> off;
        }

        self.val ^= (val >> 3) ^ (val << 5) ^ (val << 29) ^ (val >> 23);
    }

}

const MIX: u64 = 0b01010101011001010100101010101100111100101000100101011000101001011;

const PADDING: u64 = 0b1100101010100010010010101011010110110110111100100101001010100101;

impl Hasher for H1 {
    fn finish(&self) -> u64 {
        self.val
    }

    fn write(&mut self, bytes: &[u8]) {
        for i in 0..(bytes.len() / 8) {
            let val = {
                let mut val = 0;
                for x in 0..8 {
                    val |= (bytes[8 * i] as u64) << (x * 8);
                }
                val
            };
            
            self.write_part(val);
        }
        if bytes.len() % 8 != 0 {
            let val = {
                let mut val = 0;
                let present = bytes.len() - (bytes.len() / 8 * 8);
                for x in (bytes.len() / 8 * 8)..bytes.len() {
                    val |= (bytes[x] as u64) << ((x % 8) * 8);
                }
                for i in present..(8 - present) {
                    val |= PADDING & ((u8::MAX as u64) << (8 * i));
                }
                val
            };

            // FIXME: get last bytes and extend with padding
            for i in (bytes.len() / 8 * 8)..bytes.len() {
                let byte = bytes[i];
                // ensure zeros aren't making our resulting hash 0
                let val = (byte % 62) + 1;
                if val % 2 == 0 {
                    self.val ^= self.val << val;
                } else {
                    self.val ^= self.val >> val;
                }
            }
        }
    }
}
