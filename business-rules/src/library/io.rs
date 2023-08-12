use std::{
	collections::BTreeSet,
	fs,
	fs::read_to_string,
	io,
	io::{stdin, stdout, Write},
	path::{Path, PathBuf}
};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string_pretty};

use super::{
	printing::NeatPrintable,
	types::{Purchase, Rule}
};

#[derive(Debug)]
pub struct ApplicationData {
	pub purchase_path: PathBuf,
	pub rule_path:     PathBuf
}
impl ApplicationData {
	pub fn from_src_path(path: impl AsRef<str>) -> ApplicationData {
		let src_path = path.as_ref();
		ApplicationData {
			purchase_path: (String::from(src_path) + "all_purchases.json").into(),
			rule_path:     (String::from(src_path) + "all_rules.json").into()
		}
	}
}

pub trait PathFindable {
	fn get_path(data: &ApplicationData) -> &Path;
}
impl PathFindable for Purchase {
	fn get_path(data: &ApplicationData) -> &Path { data.purchase_path.as_path() }
}
impl PathFindable for Rule {
	fn get_path(data: &ApplicationData) -> &Path { data.rule_path.as_path() }
}

fn load_set<T: DeserializeOwned + Default>(path: &Path) -> io::Result<T> {
	let data_string: String = read_to_string(path)?;
	if data_string.is_empty() {
		Ok(T::default())
	} else {
		Ok(from_str(data_string.as_str())?)
	}
}
fn save_overwrite_path<T: Serialize>(data: &T, path: &Path) -> io::Result<()> {
	let data_string = to_string_pretty(&data).expect("should always be able to parse");
	fs::write(path, data_string)
}

pub(crate) trait Saved
where
	Self: NeatPrintable + Ord + Clone + Serialize + DeserializeOwned
{
	fn load_from_disk_retrying(path: &Path) -> io::Result<BTreeSet<Self>> {
		'attempt_loop: loop {
			let load_result = load_set::<BTreeSet<Self>>(path);
			if load_result.is_ok()
				|| !prompt_yes_no_question(format!(
					"Attempt to load {} failed. Do you want to try again?",
					Self::type_name_pretty()
				)) {
				break 'attempt_loop load_result;
			}
		}
	}
	fn save_to_disk_retrying(path: &Path, set: BTreeSet<Self>) -> io::Result<()> {
		'attempt_loop: loop {
			let save_result = save_overwrite_path(&set, path);
			if save_result.is_ok()
				|| !prompt_yes_no_question(format!(
					"Attempt to save {} failed. Do you want to try again?",
					Self::type_name_pretty()
				)) {
				break 'attempt_loop save_result;
			}
		}
	}
}
impl Saved for Purchase {}
impl Saved for Rule {}
pub(crate) fn prompt_yes_no_question(question: impl AsRef<str>) -> bool {
	println!("{} (Y/N)", question.as_ref());
	get_yes_no_reply()
}
pub(crate) fn get_yes_no_reply() -> bool {
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
