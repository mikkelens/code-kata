use std::{env, ops::RangeInclusive, str::FromStr};

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	if args.is_empty() {
		println!("No arguments provided.");
		return;
	}

	let (digit_count_args, calc_fn) = {
		let mut unknown_args = args;
		let method =
			if let Ok(specific_method) = unknown_args.first().unwrap().parse::<CalcMethod>() {
				println!("Method of calculation: {specific_method:?}.");
				unknown_args.remove(0);
				specific_method
			} else {
				println!("Method of calculation not provided.");
				CalcMethod::Naive
			};
		(unknown_args, match method {
			CalcMethod::Naive => calc_naive,
			CalcMethod::Restacking => calc_restacking
		})
	};

	println!("Args: {}\n", digit_count_args.join(", "));

	let mut digit_counts_to_try: Vec<usize> = Vec::new();
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
			for digit_count in range {
				if digit_count == 0 {
					continue;
				}
				digit_counts_to_try.push(digit_count);
			}
		} else {
			let digit_count: usize = arg.parse().expect("invalid number");
			if digit_count == 0 {
				continue;
			}
			digit_counts_to_try.push(digit_count);
		}
	}

	if digit_counts_to_try.is_empty() {
		println!("(No non-zero digit counts provided)");
	} else {
		for digit_count in digit_counts_to_try {
			print!("Numbers in {digit_count} binary digits, that do not have adjacent 1's: ");
			let result = calc_fn(digit_count);
			println!("{result}");
		}
	}
}
#[derive(Debug)]
enum CalcMethod {
	Naive,
	Restacking
}
impl FromStr for CalcMethod {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.to_lowercase();
		if s.contains("naive") {
			Ok(CalcMethod::Naive)
		} else if s.contains("stack") {
			Ok(CalcMethod::Restacking)
		} else {
			Err("".into())
		}
	}
}

#[allow(unused)]
fn calc_naive(digit_count: usize) -> usize {
	let digit_range = 0..digit_count;

	let upper_bound = 2usize.pow(digit_count as u32);
	let range_of_ints_in_digits = 1..upper_bound;
	// dbg!(&range_of_ints_in_digits);

	let mut binary_without_adjacent = 1; // since 0 will always count
	'numbers: for num in range_of_ints_in_digits {
		// println!("\nNew num: {num} | {num:b}");
		let mut just_saw_1 = false;
		for digit_n in digit_range.clone() {
			let bit = (num >> digit_n) & 1;
			// dbg!(digit_n, bit);
			if bit == 1 {
				if just_saw_1 {
					continue 'numbers; // try next
				} else {
					just_saw_1 = true;
				}
			} else {
				just_saw_1 = false;
			}
		}
		// println!("Num {num} ({num:b}) is valid.");
		binary_without_adjacent += 1; // valid combination
	}
	binary_without_adjacent
}
#[allow(unused)]
fn calc_restacking(digit_count: usize) -> usize {
	let mut prev = 1;
	let mut current = 2;

	for _ in 2..=digit_count {
		let new = current + prev;
		prev = current;
		current = new;
	}

	current
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn test_naive_known() {
		// 3 digits means 3 combinations with no adjacent 1's
		assert_eq!(calc_naive(1), 2); // 0 and 1
		assert_eq!(calc_naive(2), 3); // 0, 1, 2
		assert_eq!(calc_naive(3), 5); // 0, 1, 2, 4, 5
	}
	#[test]
	fn test_reusing_known() {
		// 3 digits means 3 combinations with no adjacent 1's
		assert_eq!(calc_restacking(1), 2); // 0 and 1
		assert_eq!(calc_restacking(2), 3); // 0, 1, 2
		assert_eq!(calc_restacking(3), 5); // 0, 1, 2, 4, 5
	}
	#[test]
	fn compare_reusing_with_naive_to_16_digits() {
		for digit_count in 3..=16 {
			assert_eq!(calc_naive(digit_count), calc_restacking(digit_count));
		}
	}
}
