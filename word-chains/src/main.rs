use std::{collections::HashSet, env};

const WORDLIST: &str = include_str!("wordlist.txt");

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	let (first, second) = match args.len() {
		2 => {
			let mut words = args.into_iter();
			(
				words.next().expect("First missing?"),
				words.next().expect("Second missing?")
			)
		},
		wrong_arg_count => {
			if wrong_arg_count < 2 {
				eprintln!("Not enough arguments specified. Amount required: 2");
			} else {
				eprintln!(
					"Too many arguments specified [{}]. Amount required: 2",
					wrong_arg_count
				);
			}
			return;
		}
	};
	let all_words: HashSet<&str> = WORDLIST.lines().collect();
	if let Some(path) = shortest_word_chains_recursive(&vec![], &None, &first, &second, &all_words)
	{
		print!("Shortest found path from '{}' to '{}': ", first, second);
		display_path(&path);
	} else {
		eprintln!(
			"No way to traverse from '{}' to '{}', according to this algorithm.",
			first, second
		);
	}
}
fn display_path(path: &WordPath) -> String { format!("[\n    {}\n]", path.join(",\n    ")) }

type WordPath = Vec<String>;
fn shortest_word_chains_recursive(
	chain_history_ref: &WordPath,
	shortest_path_ref: &Option<WordPath>,
	current_word: &str,
	target_word_ref: &str,
	dictionary_ref: &HashSet<&str>
) -> Option<WordPath> {
	assert_eq!(current_word.len(), target_word_ref.len());
	let chain_with_word = {
		let mut new_chain_untill_now = chain_history_ref.clone();
		dbg!(&current_word);
		new_chain_untill_now.push(current_word.to_string());
		new_chain_untill_now
	};
	eprintln!("Chain untill this point: [{}]", chain_with_word.join(", "));
	if current_word == target_word_ref {
		return Some(chain_with_word); // word reached, a path can be returned
	}
	if shortest_path_ref
		.as_ref()
		.is_some_and(|shortest| chain_with_word.len() >= shortest.len())
		|| chain_history_ref.len() > target_word_ref.len() * 2
	{
		return None;
	}

	let current_chars: Vec<char> = current_word.chars().collect();
	let target_chars: Vec<char> = target_word_ref.chars().collect();

	eprintln!("Trying the straight forward ways towards target word...");
	for i in 0..target_word_ref.len() {
		let new_word = {
			let mut new_chars = current_chars.clone();
			new_chars[i] = target_chars[i];
			new_chars.into_iter().collect::<String>()
		};
		// check if exact match
		if new_word == target_word_ref {
			eprintln!("Found target ({}), going back up...", new_word);
			let mut succesful_chain = chain_with_word;
			succesful_chain.push(new_word);
			return Some(succesful_chain);
		}
		// check if this branch is already tried/active
		if chain_with_word.contains(&new_word) {
			continue;
		}
		// continue
		if dictionary_ref.contains(new_word.as_str()) {
			if let Some(new_path) = shortest_word_chains_recursive(
				&chain_with_word,
				shortest_path_ref,
				&new_word,
				target_word_ref,
				dictionary_ref
			) {
				if chain_history_ref.len() > 1 {
					eprintln!(
						"Found path with length {}, returning it back up...",
						new_path.len()
					);
					return Some(new_path);
				} else if new_path.len() <= target_word_ref.len() {
					eprintln!("Found path with minimum possible steps.");
					return Some(new_path);
				}
			}
		}
	}

	eprintln!("Trying all possible directions from current word...");
	for i in 0..current_word.len() {
		for c in 'a'..='z' {
			if c == target_chars[i] {
				continue; // direct follow, we have already tried this above
			}
			let new_word = {
				let mut new_chars = current_chars.clone();
				new_chars[i] = c;
				new_chars.into_iter().collect()
			};
			if chain_with_word.contains(&new_word) {
				continue; // we have already tried this direction or nothing changed
			}
			if dictionary_ref.contains(new_word.as_str()) {
				if let Some(new_path) = shortest_word_chains_recursive(
					&chain_with_word,
					shortest_path_ref,
					&new_word,
					target_word_ref,
					dictionary_ref
				) {
					if let Some(existing_shortest) = &shortest_path_ref {
						if new_path.len() < existing_shortest.len() {
							return Some(new_path);
						}
					} else {
						return Some(new_path);
					}
				}
			}
		}
	}
	None
}

#[cfg(test)]
mod tests {
	use super::*;

	const CAT_INTO_DOG_STR: &str = include_str!("sample_cat_into_dog.txt");
	#[test]
	fn can_turn_cat_into_dog_simple_way() {
		let all_words: HashSet<&str> = WORDLIST.lines().collect();
		let known_path: WordPath = CAT_INTO_DOG_STR
			.lines()
			.map(std::string::ToString::to_string)
			.collect();
		let cat = known_path.first().unwrap();
		let dog = known_path.last().unwrap();
		let path = shortest_word_chains_recursive(&vec![], &None, cat, dog, &all_words).unwrap();
		dbg!(&path);
		dbg!(&known_path);
		assert_eq!(path.len(), known_path.len());
	}
	#[test]
	fn can_turn_lead_into_gold_within_four_steps() {
		const SHORT_PATH_LEN: usize = 4;
		let all_words: HashSet<&str> = WORDLIST.lines().collect();
		let lead = "lead";
		let gold = "gold";
		let path = shortest_word_chains_recursive(&vec![], &None, lead, gold, &all_words).unwrap();
		assert!(path.len() <= SHORT_PATH_LEN);
	}
	#[test]
	fn can_turn_ruby_into_code_within_six_steps() {
		const SHORT_PATH_LEN: usize = 6; // note: 5 is also possible
		let all_words: HashSet<&str> = WORDLIST.lines().collect();
		let lead = "ruby";
		let gold = "code";
		let path = shortest_word_chains_recursive(&vec![], &None, lead, gold, &all_words).unwrap();
		assert!(path.len() <= SHORT_PATH_LEN);
	}
}
