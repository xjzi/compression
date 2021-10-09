use criterion::{black_box, criterion_group, criterion_main, Criterion};
use compression::{self, compress, decompress};

fn criterion_benchmark(c: &mut Criterion) {
    let bytes = include_bytes!("../corpus/plrabn12.txt");

    c.bench_function("get frequencies", |b| b.iter(|| compress::get_freq(black_box(bytes))));
    let freq = compress::get_freq(bytes);

    c.bench_function("build tree", |b| b.iter(|| compress::get_tree(black_box(&freq))));
    let tree = compress::get_tree(&freq);

    c.bench_function("parse tree", |b| b.iter(|| compress::parse_tree(black_box(&tree))));
    let (buf, map) = compress::parse_tree(&tree);

    c.bench_function("map bytes", |b| b.iter(|| compress::code(black_box(&bytes.clone()), black_box(&mut buf.clone()), black_box(&map))));
    
    let mut buf = compress::compress(bytes);
    c.bench_function("deserialize tree", |b| b.iter(|| decompress::build_tree(black_box(&mut buf.clone()))));

    //Removes header from buf
    let tree = decompress::build_tree(&mut buf);
    c.bench_function("decode bitstream", |b| b.iter(|| decompress::decode(black_box(buf.clone()), black_box(&tree))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
