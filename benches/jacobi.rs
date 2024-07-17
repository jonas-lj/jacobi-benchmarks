use criterion::criterion_main;

mod jacobi_benchmarks {
    use criterion::{criterion_group, BatchSize, BenchmarkGroup, Criterion};
    use jacobi_benchmarks::{jacobi_base, jacobi_new, jacobi_taiko, jacobi_num_bigint_dig};
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
            group.bench_function(format!("Taiko/{}", size), move |b| {
                b.iter_batched(
                    || {
                        let mut a_bytes = vec![0u8; size / 8];
                        prng.fill_bytes(&mut a_bytes);

                        let mut m_bytes = vec![0u8; size / 8];
                        prng.fill_bytes(&mut m_bytes);
                        m_bytes[0] |= 1; // Ensure odd

                        let a = BigInt::from_bytes_le(num_bigint::Sign::Plus, &a_bytes);
                        let m = BigInt::from_bytes_le(num_bigint::Sign::Plus, &m_bytes);
                        (a.to_u64_digits().1, m.to_u64_digits().1)
                    },
                    |(a, m)| {
                        // TODO: change the impl to accept dynamic sizes
                        if size <= 128 { jacobi_taiko::<3>(&a, &m); }
                        else if size <= 256 { jacobi_taiko::<5>(&a, &m); }
                        else if size <= 384 { jacobi_taiko::<7>(&a, &m); }
                        else if size <= 512 { jacobi_taiko::<9>(&a, &m); }
                        else if size <= 768 { jacobi_taiko::<13>(&a, &m); }
                        else if size <= 1024 { jacobi_taiko::<17>(&a, &m); }
                        else if size <= 2048 { jacobi_taiko::<33>(&a, &m); }
                        else if size <= 3072 { jacobi_taiko::<49>(&a, &m); }
                    },
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
