use std::{collections::HashMap, env, fs, string::ToString};

use rand::prelude::*;

fn main() {
	let dir = env::current_dir().expect("Unable to get directory?");
	let mut args = env::args().skip(1);

	let target_file_local_path = args.next().expect("Target file argument missing!");
	let target_file_name = target_file_local_path.trim_start_matches(['.', '\\']);
	let target_path = dir.join(target_file_name);
	println!("Path: {:?}", &target_path);
	let s = fs::read_to_string(target_path).expect("Target path could not be read!");

	let mut word_map: HashMap<String, Vec<String>> = HashMap::new();
	// create
	let all_words: Vec<String> = s
		.split([' ', '\r', '\n'])
		.map(ToString::to_string)
		.filter(|word| !word.is_empty())
		.collect();
	let mut words_at_beginning: Vec<&String> = vec![];
	println!("WORDS IN VOCABULARY: {}", all_words.len());
	for window in all_words.windows(2) {
		let this = &window[0];
		let next = &window[1];
		if this.contains('.') {
			// println!("Added '{}' to list, because of '{}'", next, this);
			words_at_beginning.push(next);
		}
		word_map
			.entry(this.into())
			.or_insert(vec![next.into()])
			.push(next.into());
	}
	// consume
	let mut triagram = String::new();
	let mut rng = rand::thread_rng();
	let mut current_key: String = (*words_at_beginning
		.choose(&mut rng)
		.expect("Could not start chain!"))
	.to_string();
	let mut history: Vec<String> = vec![];
	loop {
		if !word_map.contains_key(&current_key) {
			if history.is_empty() {
				break;
			}
			current_key = history.pop().expect("History needs any element");
			continue;
		}

		triagram += current_key.as_str();
		triagram += " ";
		history.push(current_key.clone());

		let vec = word_map.entry((&current_key).into()).or_default();
		let random = (0..vec.len()).choose(&mut rng).unwrap();
		let new = vec.swap_remove(random);
		if vec.is_empty() {
			word_map.remove_entry(&current_key);
		}
		current_key = new;
	}
	println!("TRIAGRAM:\n\n{}\n", triagram);
}
