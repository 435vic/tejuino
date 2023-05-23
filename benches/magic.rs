use criterion::{criterion_group, criterion_main, Criterion};
use tejuino::util::PRNG;
use tejuino::types::*;
use tejuino::magic::*;

use rand::Rng;

pub fn benchmarks(c: &mut Criterion) {
    let mut single_square_group = c.benchmark_group("magic single square");

    single_square_group.bench_function("random square random seed", |b| { b.iter(|| {
        // choose a random square
        let mut rng = rand::thread_rng();
        let square = Square::from(rng.gen_range(0..64));
        assert_ne!(square, Square::InvalidSquare);
        // generate RNG based on rank
        // let seed = RNG_SEEDS[square.rank() as usize];
        // let mut rng = PRNG::new(seed);
        let mut rng = PRNG::new(rng.gen());
        find_magic(&MagicPiece::Rook, square, &mut rng)
    })});

    single_square_group.bench_function("random square RNG_SEEDS", |b| { b.iter(|| {
        // choose a random square
        let mut rng = rand::thread_rng();
        let square = Square::from(rng.gen_range(0..64));
        assert_ne!(square, Square::InvalidSquare);
        // generate RNG based on rank
        let seed = RNG_SEEDS[square.rank() as usize];
        let mut rng = PRNG::new(seed);
        find_magic(&MagicPiece::Rook, square, &mut rng)
    })});
    single_square_group.finish();


    let mut all_square_group = c.benchmark_group("magic all squares");

    all_square_group.bench_function("magic all squares", |b| b.iter(|| {
        generate_magics(&MagicPiece::Rook);
    }));
    all_square_group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
