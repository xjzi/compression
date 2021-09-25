pub mod huffman;
pub mod rle;

use bitvec::prelude::*;

pub fn wrap_bits(mut bits: BitVec<Lsb0, u8>) -> Vec<u8> {
    //Make buffer to fill in the last byte
    let missing = 8 - (bits.len() % 8);
    let mut buf = bitvec![0; missing];
    *buf.last_mut().unwrap() = true;
    buf.append(&mut bits);

    let mut bytes: Vec<u8> = Vec::new();
    for byte in buf.chunks(8) {
        let byte = byte.load::<u8>();
        bytes.push(byte);
    }

    bytes
}

pub fn unwrap_bytes(bytes: Vec<u8>) -> BitVec<Lsb0, u8> {
    let mut bits: BitVec<Lsb0, u8> = BitVec::new();
    for byte in bytes {
        let mut byte: BitVec<Lsb0, u8> = BitVec::from_element(byte);
        bits.append(&mut byte);
    }
    
    //Removes all 0 bits and the first 1 bit from the beginning
    while bits.remove(0) == false {}

    bits
}
