use std::ops::RangeInclusive;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lib::{calc_naive, calc_restacking, calc_restacking_reusing};

pub fn benchmark_implementations(c: &mut Criterion) {
	{
		const SMALL_RANGE: RangeInclusive<usize> = 0..=20;
		let mut all_small_scale = c.benchmark_group("All");
		all_small_scale.bench_function(format!("naive {SMALL_RANGE:?}").as_str(), |b| {
			b.iter(|| {
				for i in SMALL_RANGE {
					black_box(calc_naive(i));
				}
			})
		});
		all_small_scale.bench_function(format!("restacking {SMALL_RANGE:?}").as_str(), |b| {
			b.iter(|| {
				for i in SMALL_RANGE {
					black_box(calc_restacking(i));
				}
			})
		});
		all_small_scale.bench_function(
			format!("restacking & recycling {SMALL_RANGE:?}").as_str(),
			|b| {
				b.iter(|| {
					black_box(calc_restacking_reusing(&SMALL_RANGE));
				})
			}
		);
	}
	{
		const LARGE_RANGE: RangeInclusive<usize> = 20..=72;
		let mut restacking_group = c.benchmark_group("Restacking Only");
		restacking_group.bench_function(format!("restacking {LARGE_RANGE:?}").as_str(), |b| {
			b.iter(|| {
				for i in LARGE_RANGE {
					black_box(calc_restacking(i));
				}
			})
		});
		restacking_group.bench_function(
			format!("restacking & recycling {LARGE_RANGE:?}").as_str(),
			|b| {
				b.iter(|| {
					black_box(calc_restacking_reusing(&LARGE_RANGE));
				})
			}
		);
	}
}
criterion_group!(benches, benchmark_implementations);
criterion_main!(benches);
