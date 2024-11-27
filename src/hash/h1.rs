use std::{hash::Hasher, mem::size_of};

pub struct H1 {
    val: u64,
    buffer: u64,
    buf_idx: usize,
}

impl H1 {

    pub const fn new() -> Self {
        Self {
            val: MIX,
            buffer: 0,
            buf_idx: 0,
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

    fn write_partial<const RESPECT_BUF: bool>(&mut self, bytes: &[u8]) {
        let off = if RESPECT_BUF { self.buf_idx } else { 0 };
        let cnt = (off + bytes.len()).min(BLOCK_SIZE);
        for i in off..cnt {
            self.buffer |= (bytes[i - off] as u64) << (i * 8);
        }
        self.buf_idx += cnt;
    }

}

const MIX: u64 = 0b01010101011001010100101010101100111100101000100101011000101001011;

const PADDING: u64 = 0b1100101010100010010010101011010110110110111100100101001010100101;

const BLOCK_SIZE: usize = size_of::<u64>();

impl Hasher for H1 {
    fn finish(&self) -> u64 {
        if self.buf_idx != 0 {
            let buf = self.buffer | PADDING & ((1 << (self.buf_idx * 8)) - 1);
            let mut hasher = H1 { val: self.val, buffer: 0, buf_idx: 0 };
            hasher.write_part(buf);
            return hasher.val;
        }
        self.val
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut off = 0;
        if self.buf_idx != 0 {
            let prev = self.buf_idx;
            self.write_partial::<true>(bytes);
            off = self.buf_idx - prev;
        }
        if off != bytes.len() {
            if self.buf_idx != 0 {
                self.write_part(self.buffer);
                self.buf_idx = 0;
            }
            for block_idx in 0..(bytes.len() / BLOCK_SIZE) {
                self.write_part(u64_from_buf(&bytes[(off + block_idx * BLOCK_SIZE)..]));
            }
        }
        self.write_partial::<false>(&bytes[((bytes.len() - off) % BLOCK_SIZE)..]);
    }
}

fn u64_from_buf(buf: &[u8]) -> u64 {
    buf[0] as u64 | ((buf[1] as u64) << 8) | ((buf[2] as u64) << 16) | ((buf[3] as u64) << 24) | ((buf[4] as u64) << 32) | ((buf[5] as u64) << 40)
    | ((buf[6] as u64) << 48) | ((buf[7] as u64) << 56)
}

fn rotate_u64(val: u64, by: usize) -> u64 {
    let shift = u64::BITS as usize - by;
    (val << by) | (((val & (((1 << by) - 1) << shift))) >> shift)
}
