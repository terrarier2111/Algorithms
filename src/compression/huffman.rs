use std::{collections::{BinaryHeap, HashMap}, fmt::{Debug, Write}, hash::Hash};

pub fn encode(src: &str) -> Option<HuffmanEncoded> {
    if src.is_empty() {
        return None;
    }
    let mut occurrances = HashMap::new();
    for c in src.chars() {
        *occurrances.entry(c).or_default() += 1;
    }

    let mut ordered = {
        let mut ordered = BinaryHeap::new();
        for (chr, count) in occurrances.into_iter() {
            ordered.push(PrioritzedNode { priority: count, node: CodeNode::Leaf(chr) });
        }
        ordered
    };
    while ordered.len() > 1 {
        let lower_prio = ordered.pop().unwrap();
        let higher_prio = ordered.pop().unwrap();
        ordered.push(PrioritzedNode { priority: lower_prio.priority + higher_prio.priority, node: CodeNode::Branch { lhs: Box::new(lower_prio.node), rhs: Box::new(higher_prio.node) } });
    }
    let encoding_tree = ordered.pop().unwrap();
    let mut huff_table = HashMap::new();
    build_entries_iter(&mut huff_table, &encoding_tree.node);
    
    let mut output = BitStream::new();
    for c in src.chars() {
        output.write_array(huff_table.get(&c).as_ref().unwrap());
    }
    Some(HuffmanEncoded { table: encoding_tree.node, blob: output.into_bit_array() })
}

pub fn decode(src: HuffmanEncoded) -> String {
    let mut stream = BitStream::from_array(src.blob);
    let mut out = String::new();
    while stream.is_readable() {
        let mut c_node = &src.table;
        loop {
            match c_node {
                CodeNode::Leaf(val) => {
                    out.push(*val);
                    break;
                },
                CodeNode::Branch { lhs, rhs } => {
                    if stream.read_bit().unwrap() == 1 {
                        c_node = &rhs;
                    } else {
                        c_node = &lhs;
                    }
                },
            }
        }
    }
    out
}

/// iterative DFS impl
fn build_entries_iter(table: &mut HashMap<char, BitArray>, root_node: &CodeNode) {
    let mut nodes = vec![(root_node, BitStream::new())];
    while let Some((node, mut stream)) = nodes.pop() {
        match node {
            CodeNode::Leaf(val) => {
                table.insert(*val, stream.into_bit_array());
            },
            CodeNode::Branch { lhs, rhs } => {
                let mut lhs_stream = stream.clone();
                lhs_stream.write_bit(0);
                nodes.push((&lhs, lhs_stream));
                stream.write_bit(1);
                nodes.push((&rhs, stream));
            },
        }
    }
}

/// recursive DFS impl
fn build_entries(table: &mut HashMap<char, BitArray>, node: &CodeNode, curr_word: &mut BitStream) {
    match node {
        CodeNode::Leaf(val) => {
            table.insert(*val, curr_word.clone().into_bit_array());
        },
        CodeNode::Branch { lhs, rhs } => {
            curr_word.write_bit(1);
            build_entries(table, &rhs, curr_word);
            curr_word.pop_bit();
            curr_word.write_bit(0);
            build_entries(table, &lhs, curr_word);
            curr_word.pop_bit();
        },
    }
}

struct BitArray {
    inner: Vec<u8>,
    last_bits: u8,
}

impl Debug for BitArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.inner.is_empty() {
            return Ok(());
        }
        for byte_idx in 0..(self.inner.len() - 1) {
            for bit_idx in 0..8 {
                let bit = self.inner[byte_idx] & (1 << bit_idx);
                if bit == 0 {
                    f.write_char('0')?;
                } else {
                    f.write_char('1')?;
                }
            }
        }
        let last_byte = self.inner.last().cloned().unwrap();
        for bit_idx in 0..((self.last_bits + 8 - 1) % 8 + 1) {
            let bit = last_byte & (1 << bit_idx);
            if bit == 0 {
                f.write_char('0')?;
            } else {
                f.write_char('1')?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
struct BitStream {
    raw: Vec<u8>,
    write_bit_idx: u8,
    reader_idx: usize,
}

impl BitStream {

    fn new() -> Self {
        Self {
            raw: vec![],
            write_bit_idx: 0,
            reader_idx: 0,
        }
    }

    fn from_array(src: BitArray) -> Self {
        Self {
            raw: src.inner,
            write_bit_idx: src.last_bits,
            reader_idx: 0,
        }
    }

    fn write_array(&mut self, bits: &BitArray) {
        if bits.inner.is_empty() {
            return;
        }
        if bits.last_bits == 0 {
            for i in 0..bits.inner.len() {
                self.write_bits(bits.inner[i], 8);
            }
            return;
        }
        for i in 0..(bits.inner.len() - 1) {
            self.write_bits(bits.inner[i], 8);
        }
        self.write_bits(bits.inner.last().cloned().unwrap(), bits.last_bits);
    }

    /// always writes left-to-right
    fn write_bits(&mut self, mut raw: u8, bit_cnt: u8) {
        const BITS_PER_BYTE: u8 = 8;

        if bit_cnt == 0 {
            unreachable!();
        }
        
        let new_bit_idx = (self.write_bit_idx + bit_cnt) % BITS_PER_BYTE;
        if self.write_bit_idx != 0 {
            if bit_cnt > BITS_PER_BYTE - self.write_bit_idx {
                let prefix = raw << self.write_bit_idx;
                let elem_idx = self.raw.len() - 1;
                self.raw[elem_idx] |= prefix;
                let used_bits = BITS_PER_BYTE - self.write_bit_idx;
                raw >>= used_bits;
            } else {
                let prefix = raw << self.write_bit_idx;
                let elem_idx = self.raw.len() - 1;
                self.raw[elem_idx] |= prefix;
                self.write_bit_idx = (self.write_bit_idx + bit_cnt) % BITS_PER_BYTE;
                return;
            }
        }
        self.raw.push(raw);
        self.write_bit_idx = new_bit_idx;
    }

    /// always writes left-to-right
    fn write_bit(&mut self, raw: u8) {
        const BITS_PER_BYTE: u8 = 8;
        if self.write_bit_idx != 0 {
            let prefix = raw << self.write_bit_idx;
            let elem_idx = self.raw.len() - 1;
            self.raw[elem_idx] |= prefix;
            self.write_bit_idx = (self.write_bit_idx + 1) % BITS_PER_BYTE;
            return;
        }
        self.raw.push(raw);
        self.write_bit_idx = 1;
    }

    fn pop_bit(&mut self) -> Option<u8> {
        if self.raw.is_empty() {
            return None;
        }
        if self.write_bit_idx == 1 {
            let ret = self.raw.pop().unwrap();
            self.write_bit_idx = 0;
            return Some(ret);
        }
        let elem_idx = self.raw.len() - 1;
        // this is always 1 less than the writer index, except if the writer index 0, indicating that the whole byte is written, then it will be 7
        let shift_cnt = (self.write_bit_idx + 8 - 1) % 8;
        let ret = self.raw[elem_idx] >> shift_cnt;
        self.raw[elem_idx] &= !(1 << shift_cnt);
        self.write_bit_idx = shift_cnt;
        Some(ret)
    }

    fn into_bit_array(self) -> BitArray {
        BitArray { inner: self.raw, last_bits: self.write_bit_idx }
    }

    /// always reads left-to-right, returns 1 or 0
    fn read_bit(&mut self) -> Option<u8> {
        if !self.is_readable() {
            return None;
        }
        let val = self.raw.get(self.reader_idx / 8).cloned().unwrap();
        let ret = (val >> (self.reader_idx % 8)) & 1;
        self.reader_idx += 1;
        Some(ret)
    }

    #[inline]
    fn is_readable(&self) -> bool {
        let mut bit_cnt = self.raw.len() * 8;
        if self.write_bit_idx != 0 {
            bit_cnt -= 8 - self.write_bit_idx as usize;
        }
        self.reader_idx < bit_cnt
    }

}

#[derive(PartialEq, Eq, Debug)]
struct PrioritzedNode {
    priority: u64,
    node: CodeNode,
}

impl PartialOrd for PrioritzedNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioritzedNode {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
enum CodeNode {
    Leaf(char),
    Branch {
        lhs: Box<CodeNode>,
        rhs: Box<CodeNode>,
    }
}

#[derive(Debug)]
pub struct HuffmanEncoded {
    table: CodeNode,
    blob: BitArray,
}
