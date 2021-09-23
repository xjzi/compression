use compression::huffman;

fn main() {
    let text = include_bytes!("../corpus/alice.txt");
    let freq = compression::get_freq(text);
    huffman::compress(&freq);
}

