use criterion::{black_box, criterion_group, criterion_main, Criterion};
use compression::huffman;

fn criterion_benchmark(c: &mut Criterion) {
    let text = include_bytes!("../corpus/bible.txt");
    let freq = compression::get_freq(text);
    c.bench_function("huffman bible", |b| b.iter(|| huffman::get_codes(black_box(&freq))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
