use std::{
	collections::HashSet,
	env, fs,
	io::{stdin, stdout, Write}
};

use business_rules::{Category, Physicality, Purchase};
use strum::IntoEnumIterator;

fn main() {
	println!("FIRST ARG: {}\n", env::args().next().unwrap());

	'program_loop: loop {
		println!("What do you want to do?");
		println!(" - [C] Create a new purchase item");
		println!(" - [D] Delete an item");
		// println!(" - [U] Undo last operation");
		println!(" - [P] Print item processing steps");
		println!(" - [E] Exit");

		'input_loop: loop {
			let user_str = get_reply().to_lowercase(); // blocking IO

			if user_str.contains('c') {
				println!();
				let new_item = Purchase {
					title:       prompt_question("Provide a title to the purchase."), // any string
					physicality: request_physicality_answer(),
					category:    request_category_answer()
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
			// } else if user_str.contains('u') {
			// 	unimplemented!();
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
fn request_category_answer() -> Category {
	let categories = Category::iter()
		.map(|c| format!("{c:?}"))
		.collect::<Vec<String>>();
	println!(
		"What category of thing is it? [{:?}]",
		categories.join(", ")
	);
	loop {
		let unsure_answer = match get_reply().to_lowercase().as_str() {
			"upgrade" => Some(Category::MembershipUpgrade),
			"membership" => Some(Category::Membership),
			"book" => Some(Category::Book),
			"video" => Some(Category::Video),
			_ => None
		};
		if let Some(valid_answer) = unsure_answer {
			break valid_answer;
		}
		println!("Your answer was not recognized as a valid category. Please try again.");
	}
}
fn request_physicality_answer() -> Physicality {
	match promt_bool_question("Is it a physical thing? (Y/N)") {
		true => Physicality::Physical,
		false => Physicality::Nonphysical
	}
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
		let physicality_can_distinguish = found_matches
			.iter()
			.any(|item| item.physicality == Physicality::Physical)
			&& found_matches
				.iter()
				.any(|item| item.physicality == Physicality::Physical);
		if physicality_can_distinguish {
			found_matches = filter_by_physicality(found_matches);
			if found_matches.len() > 1 {
				println!("There are multiple items with the same name *and* physicality.");
				found_matches = filter_by_category(found_matches);
				// category will distinguish perfectly if physicality does not
			}
			Some(found_matches.remove(0))
		} else {
			// assume hashset correctness
			found_matches = filter_by_category(found_matches);
			// category will distinguish perfectly if physicality does not
			Some(found_matches.remove(0))
		}
	}
}
fn filter_by_physicality(found_matches: Vec<Purchase>) -> Vec<Purchase> {
	let physicality = request_physicality_answer();
	found_matches
		.into_iter()
		.filter(|item| item.physicality == physicality)
		.collect()
}
fn filter_by_category(found_matches: Vec<Purchase>) -> Vec<Purchase> {
	let category = request_category_answer();
	found_matches
		.into_iter()
		.filter(|item| item.category == category)
		.collect()
}

const DATA_PATH: &str = "F:/Git/Rust/code-kata/business-rules/src/all_items.json";
fn load_purchases() -> HashSet<Purchase> {
	let data_string = fs::read_to_string(DATA_PATH).expect("could not read from file");
	if data_string.is_empty() {
		HashSet::new()
	} else {
		serde_json::from_str(data_string.as_str()).expect("could not parse")
	}
}
fn save_purchases_overwrite(purchases: HashSet<Purchase>) {
	let string = serde_json::to_string_pretty(&purchases).expect("could not parse");
	fs::write(DATA_PATH, string).expect("unable to write to file");
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
