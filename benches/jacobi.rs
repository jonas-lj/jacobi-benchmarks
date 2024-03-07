use criterion::criterion_main;

mod jacobi_benchmarks {
    use criterion::{criterion_group, BatchSize, BenchmarkGroup, Criterion};
    use jacobi_benchmarks::{jacobi_base, jacobi_new, jacobi_num_bigint_dig};
    use num_bigint::BigInt;
    use rand::rngs::ThreadRng;
    use rand::{thread_rng, RngCore};
    criterion_group! {
        name = jacobi_benchmarks;
        config = Criterion::default().sample_size(100);
        targets = jacobi,
    }

    fn jacobi(c: &mut Criterion) {
        static BATCH_SIZES: [usize; 8] = [128, 256, 384, 512, 768, 1024, 2048, 3072];
        let mut group: BenchmarkGroup<_> = c.benchmark_group("Jacobi");

        for &size in &BATCH_SIZES {
            let mut prng: ThreadRng = thread_rng();
            group.bench_function(format!("Base/{}", size), move |b| {
                b.iter_batched(
                    || {
                        let mut a_bytes = vec![0u8; size / 8];
                        prng.fill_bytes(&mut a_bytes);

                        let mut m_bytes = vec![0u8; size / 8];
                        prng.fill_bytes(&mut m_bytes);
                        m_bytes[0] |= 1; // Ensure odd

                        let a = BigInt::from_bytes_le(num_bigint::Sign::Plus, &a_bytes);
                        let m = BigInt::from_bytes_le(num_bigint::Sign::Plus, &m_bytes);
                        (a, m)
                    },
                    |(a, m)| jacobi_base(&a, &m),
                    BatchSize::SmallInput,
                )
            });

            let mut prng: ThreadRng = thread_rng();
            group.bench_function(format!("New/{}", size), move |b| {
                b.iter_batched(
                    || {
                        let mut a_bytes = vec![0u8; size / 8];
                        prng.fill_bytes(&mut a_bytes);

                        let mut m_bytes = vec![0u8; size / 8];
                        prng.fill_bytes(&mut m_bytes);
                        m_bytes[0] |= 1; // Ensure odd

                        let a = BigInt::from_bytes_le(num_bigint::Sign::Plus, &a_bytes);
                        let m = BigInt::from_bytes_le(num_bigint::Sign::Plus, &m_bytes);
                        (a, m)
                    },
                    |(a, m)| jacobi_new(&a, &m),
                    BatchSize::SmallInput,
                )
            });

            let mut prng: ThreadRng = thread_rng();
            group.bench_function(format!("num-bigint-dig/{}", size), move |b| {
                b.iter_batched(
                    || {
                        let mut a_bytes = vec![0u8; size / 8];
                        prng.fill_bytes(&mut a_bytes);

                        let mut m_bytes = vec![0u8; size / 8];
                        prng.fill_bytes(&mut m_bytes);
                        m_bytes[0] |= 1; // Ensure odd

                        let a = num_bigint_dig::BigInt::from_bytes_le(
                            num_bigint_dig::Sign::Plus,
                            &a_bytes,
                        );
                        let m = num_bigint_dig::BigInt::from_bytes_le(
                            num_bigint_dig::Sign::Plus,
                            &m_bytes,
                        );
                        (a, m)
                    },
                    |(a, m)| jacobi_num_bigint_dig(&a, &m),
                    BatchSize::SmallInput,
                )
            });
        }
    }
}

criterion_main!(jacobi_benchmarks::jacobi_benchmarks,);
