use std::{env, ops::RangeInclusive};

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	if args.is_empty() {
		println!("No arguments provided.");
		return;
	}

	let (digit_count_args, calc_method) = {
		let mut unknown_args = args;
		let method: lib::CalcApproach = {
			if let Ok(specific_method) = unknown_args.first().unwrap().parse::<lib::CalcApproach>()
			{
				println!("Method of calculation: {specific_method:?}.");
				unknown_args.remove(0);
				specific_method
			} else {
				println!("Method of calculation not provided.");
				lib::CalcApproach::Single(lib::SingleCalc::Naive)
			}
		};
		(unknown_args, method)
	};

	println!("Args: {}", digit_count_args.join(", "));

	let mut digit_counts_ranges: Vec<RangeInclusive<usize>> = Vec::new();
	for arg in &digit_count_args {
		if arg.contains("..") {
			let range: RangeInclusive<usize> = {
				if arg.contains("..=") {
					let split = arg.split_once("..=").unwrap();
					let num1 = split.0.parse().expect("invalid number 1");
					let num2 = split.1.parse().expect("invalid number 2");
					num1..=num2
				} else {
					let split = arg.split_once("..").unwrap();
					let num1 = split.0.parse().expect("invalid number 1");
					let num2: usize = split.1.parse().expect("invalid number 2");
					num1..=(num2 - 1)
				}
			};
			digit_counts_ranges.push(range);
		} else {
			let digit_count: usize = arg.parse().expect("invalid number/range");
			if digit_count == 0 {
				continue;
			}
			digit_counts_ranges.push(1..=digit_count);
		}
	}

	if digit_counts_ranges.is_empty() {
		println!("(No non-zero digit counts provided)");
	} else {
		match calc_method {
			lib::CalcApproach::Single(single_method) => {
				let single_fn = match single_method {
					lib::SingleCalc::Naive => lib::calc_naive,
					lib::SingleCalc::Restacking => lib::calc_restacking
				};
				for range in digit_counts_ranges {
					for digit_count in range {
						let result = single_fn(digit_count);
						print!(
							"Numbers in {} binary digits, with no adjacent 1's: {}",
							digit_count, result
						);
					}
				}
			},
			lib::CalcApproach::Multi(multi_method) => {
				let multi_fn = match multi_method {
					lib::MultiCalc::RestackingAndReusing => lib::calc_restacking_reusing
				};
				for range in digit_counts_ranges {
					let range_results = multi_fn(&range);
					for (digit_count, result) in range.zip(range_results) {
						println!(
							"Numbers in {} binary digits, with no adjacent 1's: {}",
							digit_count, result
						);
					}
				}
			}
		}
	}
}
