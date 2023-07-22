use std::{
	collections::{BTreeSet, HashSet},
	env, fs,
	io::{stdin, stdout, Write}
};

use business_rules::{Purchase, Rule, TagCollection};

fn main() {
	println!("FIRST ARG: {}\n", env::args().next().unwrap());

	'program_loop: loop {
		println!("What do you want to do?");
		println!(" - [C] Create a new purchase item");
		println!(" - [D] Delete an item");
		println!(" - [P] Print item processing steps");
		println!(" - [E] Exit");

		'input_loop: loop {
			let user_str = get_reply().to_lowercase(); // blocking IO

			if user_str.contains('c') {
				println!();
				let new_item = Purchase {
					title:       prompt_question("Provide a title to the purchase."), // any string
					identifiers: request_identifiers_answer()
				};
				let mut items = load_purchases();
				if items.insert(new_item) {
					save_purchases_overwrite(items);
					println!("\nSaved item into dataset.")
				} else {
					println!("\nThis exact item already exists in the dataset, skipping saving.");
				}
			} else if user_str.contains('d') {
				println!();
				println!("In order to delete an item we must first find it.");
				let items = load_purchases();
				let mut updated_items = items.clone();
				let possible_item = quick_find_item(items.into_iter().collect());
				if let Some(existing_item) = possible_item {
					let confirmation_question =
						format!("Are you sure you want to delete...\n{existing_item:?}\n...?");
					if promt_bool_question(&confirmation_question) {
						updated_items.remove(&existing_item);
						save_purchases_overwrite(updated_items);
						println!("\nItem was removed from dataset.")
					} else {
						println!("\nItem was kept in dataset.");
					}
				} else {
					println!("\nNo item with the provided specifications could be found.");
				}
			} else if user_str.contains('p') {
				println!();
				let items = load_purchases();
				let possible_item = quick_find_item(items.into_iter().collect());
				if let Some(existing_item) = possible_item {
					println!(
						"\nThe processing steps for this items are the following:\n - {}",
						existing_item.get_processing_steps().join("\n - ")
					)
				} else {
					println!("No item with the provided specifications could be found.");
				}
			} else if user_str.contains('e') {
				break 'program_loop;
			} else {
				println!("You typed an unrecognized command. Try using the letters in '[]' above.");
				continue 'input_loop;
			}
			break 'input_loop;
		}
		println!(); // completed command operation, space for effect
	}
}

fn request_identifiers_answer() -> TagCollection {
	println!("Please provide some tags to identify the entry (separated by semicolon).");
	let mut identifiers = TagCollection::default();
	'modify_loop: loop {
		// always start by adding
		let reply = get_reply();
		let identifiers_to_add = reply.split(';').map(|s| s.trim()).collect::<Vec<_>>();
		for identifier in identifiers_to_add {
			if !identifiers.insert(identifier.into()) {
				println!(
					"'{}' is already an identifier for this entry, skipping addition...",
					identifier
				);
			}
		}
		'review_loop: loop {
			println!(
				"Identifiers: ['{}']",
				identifiers
					.clone()
					.into_iter()
					.collect::<Vec<_>>()
					.join("', '")
			);
			if promt_bool_question("Are you satisfied with the identifiers?") {
				break 'modify_loop; // finish and return identifiers
			} else {
				println!("Do you want to add [A] or delete [D] modifiers? ('C' to cancel)");
				'operation_loop: loop {
					let reply = get_reply().to_lowercase();
					if reply.contains('a') {
						println!("What do you want to add? (Still separated by semicolon)");
						continue 'modify_loop; // reuse start
					} else if reply.contains('d') {
						println!("What do you want to delete? (Still separated by semicolon)");
						let reply = get_reply();
						let identifiers_for_removal =
							reply.split(';').map(|s| s.trim()).collect::<Vec<&str>>();
						for identifier in identifiers_for_removal {
							if !identifiers.remove(identifier) {
								println!(
									"'{}' is not an identifier for this entry, skipping removal...",
									identifier
								);
							}
						}
						continue 'review_loop; // modify complete
					} else {
						println!("You must use one of the key letters above to signal intent.");
						continue 'operation_loop; // try input again
					}
				}
			}
		}
	}
	identifiers
}
fn quick_find_item(data: Vec<Purchase>) -> Option<Purchase> {
	let title = prompt_question("What is the title of the item?");
	let mut found_matches = Vec::new();
	for item in data {
		if item.title == title {
			found_matches.push(item);
		}
	}
	if found_matches.is_empty() {
		None
	} else if found_matches.len() == 1 {
		Some(found_matches.remove(0))
	} else {
		println!("Multiple items with the same name was found in the dataset.");
		let unique_tags = {
			let mut tags: HashSet<String> = HashSet::new();
			for found_match in found_matches {
				for tag in found_match.get_all_tags() {
					tags.insert(tag.to_string());
				}
			}
			tags
		};
		let tag_list: Vec<_> = unique_tags.into_iter().collect();
		println!(
			"Tags present for entries with this name: [{}] (unordered)",
			tag_list.join(", ")
		);

		todo!()
	}
}

const ITEM_DATA_PATH: &str = "F:/Git/Rust/code-kata/business-rules/src/all_items.json";
fn load_purchases() -> BTreeSet<Purchase> {
	let data_string = fs::read_to_string(ITEM_DATA_PATH).expect("could not read from file");
	if data_string.is_empty() {
		BTreeSet::new()
	} else {
		serde_json::from_str(data_string.as_str()).expect("could not parse")
	}
}
fn save_purchases_overwrite(purchases: BTreeSet<Purchase>) {
	let string = serde_json::to_string_pretty(&purchases).expect("could not parse");
	fs::write(ITEM_DATA_PATH, string).expect("unable to write to file");
}

const RULE_DATA_PATH: &str = "F:/Git/Rust/code-kata/business-rules/src/all_rules.json";
fn load_rules() -> BTreeSet<Rule> {
	let data_string = fs::read_to_string(RULE_DATA_PATH).expect("could not read from file");
	if data_string.is_empty() {
		BTreeSet::new()
	} else {
		serde_json::from_str(data_string.as_str()).expect("could not parse")
	}
}
fn save_rules_overwrite(rules: BTreeSet<Rule>) {
	let data_string = serde_json::to_string_pretty(&rules).expect("could not parse");
	fs::write(ITEM_DATA_PATH, data_string).expect("unable to write to file");
}

fn promt_bool_question(question: &str) -> bool {
	println!("{}", question);
	loop {
		let reply = get_reply().to_lowercase();
		let unsure_answer = if reply.contains('y') {
			Some(true)
		} else if reply.contains('n') {
			Some(false)
		} else {
			None
		};
		if let Some(valid_answer) = unsure_answer {
			break valid_answer;
		}
		println!("You need to answer with a yes [Y] or no [N].")
	}
}
fn prompt_question(question: &str) -> String {
	println!("{question}");
	get_reply()
}
fn get_reply() -> String {
	print!("> ");
	// flush enables us to write without a newline and have it display pre-input
	stdout().flush().expect("flush failed");
	read_line().trim().to_string()
}
fn read_line() -> String {
	let mut buffer = String::new();
	stdin().read_line(&mut buffer).expect("unable to read line");
	buffer
}
