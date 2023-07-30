#[must_use]
pub fn int_min_representation_size_ascending(number: usize) -> u8 {
	let mut min_bit_required: u8 = 1;
	let mut min_decimal_representation = 2;
	loop {
		if number < min_decimal_representation {
			return min_bit_required;
		}
		min_bit_required += 1;
		min_decimal_representation *= 2;
	}
}
#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn int_min_representation_size_log(raw_number: usize) -> u8 {
	// ilog2() must be shifted!
	// target: 0 -> 1 bit, 1 -> 1 bit, 2 -> 2 bits, 3 -> 2 bits, 4 -> 3 bits
	// ilog2: 0 -> (crash), 1 -> 0,    2 -> 1,      3 -> 1,      4 -> 2
	if raw_number == 0 {
		return 1;
	}
	let log_number = raw_number.ilog2() as u8;
	log_number + 1
}

#[cfg(test)]
mod tests {
	use std::mem::{size_of, size_of_val};

	use super::*;

	#[derive(Debug)]
	struct BitSizeTest {
		expected_result: u8,
		number:          usize
	}
	const BIT_NUMBER_TESTS: &[BitSizeTest] = &[
		BitSizeTest {
			expected_result: 1,
			number:          0
		},
		BitSizeTest {
			expected_result: 1,
			number:          1
		},
		BitSizeTest {
			expected_result: 2,
			number:          2
		},
		BitSizeTest {
			expected_result: 2,
			number:          3
		},
		BitSizeTest {
			expected_result: 3,
			number:          4
		},
		BitSizeTest {
			expected_result: 3,
			number:          5
		},
		BitSizeTest {
			expected_result: 3,
			number:          7
		},
		BitSizeTest {
			expected_result: 10,
			number:          1000
		},
		BitSizeTest {
			expected_result: 20,
			number:          1_000_000
		},
		BitSizeTest {
			expected_result: 30,
			number:          1_000_000_000
		},
		BitSizeTest {
			expected_result: 33,
			number:          8_000_000_000
		}
	];
	fn test_bit_size_implementation(implementation: fn(usize) -> u8) {
		for bit_number_test in BIT_NUMBER_TESTS {
			assert_eq!(
				bit_number_test.expected_result,
				implementation(bit_number_test.number)
			);
		}
	}
	#[test]
	fn bits_size_brute_test() {
		test_bit_size_implementation(int_min_representation_size_ascending);
	}
	#[test]
	fn bits_size_log_test() { test_bit_size_implementation(int_min_representation_size_log) }

	#[allow(unused)]
	#[derive(Debug)]
	struct Character {
		name:         String,
		address:      String,
		phone_number: u64
	}
	#[test]
	#[allow(clippy::inconsistent_digit_grouping)]
	fn town_residence_automatic_representation() {
		println!("Character Type Size: {}", size_of::<Character>());
		let character_example = Character {
			name:         "Onoanido Awopdnipo Gkawp Ioptranz Hansen".to_string(),
			address:      "I Sure Hope Not 56th Street no. 25".to_string(),
			phone_number: 999_55_69_42_01
		};
		println!(
			"Example Character [{:?}] value size: {}",
			character_example,
			size_of_val(&character_example)
		);
	}
}
