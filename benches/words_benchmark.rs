use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{BufRead, BufReader};

use autocomplete::Dictionary;

fn words_benchmark(c: &mut Criterion) {
    let filename = "dictionary.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let words: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let dict = Dictionary::<usize>::build_without_weights(words);

    // Benchmarking against empty prefix which is the worst case scenario in terms of building the result vec
    c.bench_function("all words", |b| {
        b.iter(|| {
            let _ = dict.words(black_box(""));
        })
    });

    // Benchmarking against long string, many lookups
    c.bench_function("starting with 'superserviceabl'", |b| {
        b.iter(|| {
            let _ = dict.words(black_box("superserviceabl"));
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = words_benchmark
}
criterion_main!(benches);
