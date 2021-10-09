use compression;
use bitvec::prelude::*;

#[test]
fn wrapper() {
    let bits: BitVec<Lsb0, u8> = bitvec![Lsb0, u8; 0, 1, 0, 1];
    let wrapped = compression::wrap_bits(bits.clone());
    let unwrapped = compression::unwrap_bytes(wrapped);
    assert_eq!(bits, unwrapped);
}

#[test]
fn huffman() {
    let original = include_bytes!("../corpus/alice29.txt");
    let compressed =  compression::compress::compress(original);
    let decompressed = compression::decompress::decompress(compressed);
    assert_eq!(original, &decompressed[..]);
}
