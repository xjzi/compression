mod code;
mod node;

use std::collections::HashMap;
use bitvec::prelude::*;

fn main() {
    let text = include_bytes!("alice.txt");
    println!("The file is {} bytes.", text.len());
    let freq = get_freq(text);
    let res = code::huffman(&freq);
    let mut res: Vec<(u8, BitVec)> = res.into_iter().collect();
    res.sort_by_key(|a| freq.get(&a.0).unwrap());

    for (val, code) in res.iter() {
        println!("{} | freq: {} code: {}.", *val as char, freq.get(val).unwrap(), code);
    }
}

fn get_freq(input: &[u8]) -> HashMap<u8, u32> {
    let mut map: HashMap<u8, u32> = HashMap::new();
    for val in input {
        let entry = map.entry(*val).or_insert(0);
        *entry = *entry + 1;
    }
    map
}

