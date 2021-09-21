pub mod huffman;
pub mod rle;

use std::collections::HashMap;

pub fn get_freq(input: &[u8]) -> HashMap<u8, u32> {
    let mut map: HashMap<u8, u32> = HashMap::new();
    for val in input {
        let entry = map.entry(*val).or_insert(0);
        *entry = *entry + 1;
    }
    map
}

