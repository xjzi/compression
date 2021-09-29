use compression::huffman;

#[test]
fn test_compress_decompress() {
    let original = include_bytes!("../corpus/alice.txt");
    let compressed =  huffman::compress::compress(original);
    let decompressed = huffman::decompress::decompress(compressed);
    assert_eq!(original, &decompressed[..]);
}
