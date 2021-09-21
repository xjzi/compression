use compression::huffman;
use bitvec::prelude::*;

fn main() {
    let text = include_bytes!("../corpus/alice.txt");

    let freq = compression::get_freq(text);
    let res = huffman::get_codes(&freq);

    let mut res: Vec<(u8, BitVec)> = res.into_iter().collect();
    res.sort_by_key(|a| freq.get(&a.0).unwrap());

    for (val, code) in res.iter() {
        println!("{} | freq: {} code: {}.", *val as char, freq.get(val).unwrap(), code);
    }
}

