use std::{ops::RangeInclusive, str::FromStr};

#[derive(Debug)]
pub enum SingleCalc {
	Naive,
	Restacking
}

#[derive(Debug)]
pub enum MultiCalc {
	RestackingAndReusing
}

#[derive(Debug)]
pub enum CalcApproach {
	Single(SingleCalc),
	Multi(MultiCalc)
}

impl FromStr for CalcApproach {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.to_lowercase();
		if s.contains("naive") {
			Ok(CalcApproach::Single(SingleCalc::Naive))
		} else if s.contains("recycl") {
			Ok(CalcApproach::Multi(MultiCalc::RestackingAndReusing))
		} else if s.contains("stack") {
			Ok(CalcApproach::Single(SingleCalc::Restacking))
		} else {
			Err("".into())
		}
	}
}

#[inline]
pub fn calc_naive(digit_count: usize) -> usize {
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

#[inline]
pub fn calc_restacking(digit_count: usize) -> usize {
	let mut prev = 1; // kicks off incrementation from zero and up
	let mut current = 2; // result of zero

	for _ in 2..=digit_count {
		let new = current + prev;
		prev = current;
		current = new;
	}

	current
}

pub type OrderedResults = Vec<usize>;

#[inline]
pub fn calc_restacking_reusing(digit_count_range: &RangeInclusive<usize>) -> OrderedResults {
	let mut prev = 1; // kicks off incrementation from zero and up
	let mut current = 2; // result of zero

	let mut results: OrderedResults = Vec::new();

	for i in 2..=*digit_count_range.end() {
		let new = current + prev;
		prev = current;
		current = new;

		if digit_count_range.contains(&i) {
			results.push(current)
		}
	}

	results
}

#[cfg(test)]
pub mod tests {
	use super::*;

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
