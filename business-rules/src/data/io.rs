use std::{
	any::type_name,
	fs,
	fs::read_to_string,
	io::{stdin, stdout, Write}
};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string_pretty};

pub fn load_set<T: DeserializeOwned + Default>(path: &str) -> T {
	'reading: loop {
		let data_string: String = read_to_string(path).expect("could not read from file");
		if data_string.is_empty() {
			break 'reading T::default();
		} else if let Ok(data) = from_str(data_string.as_str()) {
			break 'reading data;
		} else {
			println!(
				"Could not parse data as {} from path '{}'",
				type_name::<T>(),
				path
			);
			println!("Press enter to retry.");
			let _ = read_line();
		}
	}
}
pub fn save_overwrite_path<T: Serialize>(data: T, path: &str) {
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
pub fn get_reply() -> String {
	print!("> ");
	// flush enables us to write without a newline and have it display pre-input
	stdout().flush().expect("flush failed"); // possibly breaks everything in certain terminal environments
	read_line().trim().to_string()
}
pub fn read_line() -> String {
	let mut buffer = String::new();
	stdin().read_line(&mut buffer).expect("unable to read line");
	buffer
}