use std::{
	any::type_name,
	collections::BTreeSet,
	fs,
	fs::read_to_string,
	io::{stdin, stdout, Write}
};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string_pretty};

use super::types::{Purchase, Rule};

fn load_set<T: DeserializeOwned + Default>(path: &str) -> T {
	'reading: loop {
		let data_string: String = read_to_string(path).expect("could not read from file");
		if data_string.is_empty() {
			break 'reading T::default();
		} else if let Ok(data) = from_str(data_string.as_str()) {
			break 'reading data;
		}
		println!(
			"Could not parse data as {} from path '{}'",
			type_name::<T>(),
			path
		);
		println!("Press enter to retry.");
		let _ = read_line();
	}
}
fn save_overwrite_path<T: Serialize>(data: T, path: &str) {
	let data_string = to_string_pretty(&data).expect("should always be able to parse");
	'write: loop {
		if fs::write(path, data_string.clone()).is_err() {
			println!(
				"Could not write data as {} to path '{}'.",
				type_name::<T>(),
				path
			);
			println!("Press enter key to retry.");
			let _ = read_line();
		} else {
			break 'write;
		}
	}
}

pub(crate) trait Saved
where
	Self: Ord + Clone
{
	fn load_from_disk() -> BTreeSet<Self>
	where
		Self: Sized;
	fn save_to_disk(set: BTreeSet<Self>)
	where
		Self: Sized;
}
const PURCHASE_DATA_PATH: &str =
	r"C:\Users\mikke\Desktop\repos\rust\code-kata\business-rules\src\all_purchases.json";
impl Saved for Purchase {
	fn load_from_disk() -> BTreeSet<Self>
	where
		Self: Sized
	{
		load_set(PURCHASE_DATA_PATH)
	}

	fn save_to_disk(set: BTreeSet<Self>)
	where
		Self: Sized
	{
		save_overwrite_path(set, PURCHASE_DATA_PATH);
	}
}
const RULE_DATA_PATH: &str =
	r"C:\Users\mikke\Desktop\repos\rust\code-kata\business-rules\src\all_rules.json";
impl Saved for Rule {
	fn load_from_disk() -> BTreeSet<Self>
	where
		Self: Sized
	{
		load_set(RULE_DATA_PATH)
	}

	fn save_to_disk(set: BTreeSet<Self>)
	where
		Self: Sized
	{
		save_overwrite_path(set, RULE_DATA_PATH);
	}
}
pub(crate) fn get_yes_no_answer(question: impl AsRef<str>) -> bool {
	println!("{} (Y/N)", question.as_ref());
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
		println!("You need to answer with a yes [Y] or no [N].");
	}
}
pub(crate) fn prompt_question(question: impl AsRef<str>) -> String {
	println!("{}", question.as_ref());
	get_reply()
}
pub(crate) fn try_prompt_question(question: impl AsRef<str>) -> Option<String> {
	println!("{}", question.as_ref());
	try_get_reply()
}
pub(crate) fn get_reply() -> String { try_get_reply().expect("flush failed") }
pub(crate) fn try_get_reply() -> Option<String> {
	print!("> ");
	// flush enables us to write without a newline and have it display pre-input
	stdout().flush().ok()?; // possibly breaks everything in certain terminal environments
	Some(read_line().trim().to_string())
}
pub(crate) fn read_line() -> String { try_read_line().expect("unable to read line") }
pub(crate) fn try_read_line() -> Option<String> {
	let mut buffer = String::new();
	stdin().read_line(&mut buffer).ok()?;
	Some(buffer)
}
