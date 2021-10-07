use criterion::{black_box, criterion_group, criterion_main, Criterion};
use compression::huffman;

fn criterion_benchmark(c: &mut Criterion) {
    let text = include_bytes!("../corpus/alice29.txt");
    let compressed = huffman::compress::compress(text);
    c.bench_function("huffman compress alice", |b| b.iter(|| huffman::compress::compress(black_box(text))));
    c.bench_function("huffman decompress alice", |b| b.iter(|| huffman::decompress::decompress(black_box(compressed.clone()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
