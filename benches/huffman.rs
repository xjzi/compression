use criterion::{black_box, criterion_group, criterion_main, Criterion};
use compression;

fn criterion_benchmark(c: &mut Criterion) {
    let text = include_bytes!("../corpus/alice29.txt");
    let compressed = compression::compress::compress(text);
    c.bench_function("huffman compress alice", |b| b.iter(|| compression::compress::compress(black_box(text))));
    c.bench_function("huffman decompress alice", |b| b.iter(|| compression::decompress::decompress(black_box(compressed.clone()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
