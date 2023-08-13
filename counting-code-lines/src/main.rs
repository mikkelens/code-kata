use std::{
	collections::{HashMap, HashSet},
	env, fs,
	path::PathBuf
};

use phf::phf_map;

pub mod code_analysis;

struct CommandInfo {
	env_var:      &'static str,
	_description: &'static str
}
const IGNORE_DIR_KEY: &str = "ignore_dirs";
const FILE_TYPE_KEY: &str = "file_types";
const ANALYSIS_TYPE_KEY: &str = "analysis_type";
static COMMANDS: phf::Map<&'static str, CommandInfo> = phf_map! {
	"ignore_dirs" => CommandInfo { /*name: IGNORE_DIR_KEY,*/ env_var: "GLOBAL_IGNORED_DIRS", _description: "list of directory names to ignore" },
	"file_types" => CommandInfo { /*name: FILE_TYPE_KEY,*/ env_var: "GLOBAL_FILE_TYPES", _description: "list of file type extensions to search for" },
	"analysis_type" => CommandInfo { /*name: FILE_TYPE_KEY,*/ env_var: "GLOBAL_ANALYSIS_TYPE", _description: "what kind of analysis we should (descriptive, statistic)" },
};

const ENV_VAR_SPLIT: char = ';';
fn load_command_env_variables(command: &CommandInfo) -> Option<Vec<String>> {
	let command_var = env::var(command.env_var);
	let Ok(var_str) = command_var else {
		// eprintln!("Could not find {} using enviromental variable '{}'",
		// command.description, command.env_var);
		return None;
	};
	let loaded: Vec<String> = var_str
		.split(ENV_VAR_SPLIT)
		.map(|value| value.trim().to_string())
		.collect();
	// println!("Loaded environmental variables: [{}]", loaded.join(", "));
	Some(loaded)
}

fn main() {
	let dir = env::current_dir().expect("User had no environment to get directory from");
	let all_args: Vec<String> = env::args().skip(1).collect(); // skip executable path, then collect

	if all_args.is_empty() {
		println!("No args were provided, exiting (hint: give a relative or absolute path)");
		return;
	}

	println!("All args: {}", all_args.join(", "));

	let mut command_settings_map: HashMap<&'static str, Vec<String>> = HashMap::from([
		(
			IGNORE_DIR_KEY,
			load_command_env_variables(COMMANDS.get(IGNORE_DIR_KEY).unwrap()).unwrap_or_default()
		),
		(
			FILE_TYPE_KEY,
			load_command_env_variables(COMMANDS.get(FILE_TYPE_KEY).unwrap()).unwrap_or_default()
		)
	]);

	'arg_loop: for arg in &all_args {
		const LOCAL_COMMAND_ARG: &str = "--";
		const LOCAL_COMMAND_DIVIDER: char = '=';
		let arg_after_command_trim = arg.trim_start_matches(LOCAL_COMMAND_ARG);
		let is_command = arg_after_command_trim != arg;
		if is_command {
			println!("Interpreting command '{}'...", arg_after_command_trim);
			let command_str = arg_after_command_trim.to_lowercase();
			let command_split = command_str.split_once(LOCAL_COMMAND_DIVIDER);
			let Some((command_type_str, command_val_str)) = command_split else {
				println!(
					"Tried parsing argument '{}' as command, failed to see assignment (=).",
					arg
				);
				continue;
			};

			if let Some(command_key) = COMMANDS.get_key(command_type_str) {
				match command_key {
					&ANALYSIS_TYPE_KEY => {},
					&IGNORE_DIR_KEY | &FILE_TYPE_KEY => {
						command_settings_map
							.get_mut(command_key)
							.unwrap()
							.push(command_val_str.to_string());
					},
					_ => {
						println!(
							"Did not understand command '{}' (value [{}]).",
							command_type_str, command_val_str
						);
						continue 'arg_loop;
					}
				}
			}
		} else {
			let arg_path = dir.join(arg);
			let ignored_dirs = command_settings_map.get(IGNORE_DIR_KEY).unwrap().clone();
			println!(
				"Ignoring directories with names [{}]...",
				ignored_dirs.join(", ")
			);
			let file_types = command_settings_map.get(FILE_TYPE_KEY).unwrap().clone();
			if !file_types.is_empty() {
				println!(
					"Only searching for file extensions [{}]...",
					file_types.join(", ")
				);
			}
			println!();
			let i_d_map: HashSet<String> = HashSet::from_iter(ignored_dirs);
			let f_t_map: HashSet<String> = HashSet::from_iter(file_types);
			let analysis = load_command_env_variables(COMMANDS.get(ANALYSIS_TYPE_KEY).unwrap())
				.unwrap_or_default();
			if analysis.contains(&"descriptive".to_string()) {
				analyze_path_recursive(&arg_path, &i_d_map, &f_t_map);
				println!("\nProgram finished.");
			} else {
				let lines = count_lines_path_recursive(&arg_path, &i_d_map, &f_t_map);
				println!("\nProgram finished. Total lines counted: {}", lines);
			}
		}
	}
}

fn count_lines_path_recursive(
	path: &PathBuf,
	ignored_dirs: &HashSet<String>,
	file_types: &HashSet<String>
) -> u128 {
	if path.is_dir() {
		let dir_path = path;
		if let Some(dir_name_os_str) = dir_path.file_name() {
			if let Some(dir_name) = dir_name_os_str.to_str() {
				if !ignored_dirs.contains(&dir_name.to_lowercase()) {
					// println!("Ignoring directory '{}'...", dir_name);
					let Ok(paths) = fs::read_dir(dir_path) else {
						eprintln!("{} could not be read as a directory.", dir_path.display());
						return 0;
					};

					let mut lines = 0;
					for dir_entry_fallible in paths {
						let Ok(dir_entry) = dir_entry_fallible else {
							eprintln!(
								"Could not use dir_entry: {}",
								dir_entry_fallible.unwrap_err()
							);
							continue;
						};
						let path = dir_entry.path();
						lines += count_lines_path_recursive(&path, ignored_dirs, file_types);
					}
					return lines;
				}
			}
		}
	} else if path.is_file() {
		let file_path = path;
		if file_types.is_empty() {
			return count_lines_in_file(file_path);
		} else if let Some(file_extension_os_str) = file_path.extension() {
			if let Some(file_extension) = file_extension_os_str.to_str() {
				if file_types.contains(&file_extension.to_lowercase()) {
					// println!("Ignoring file extension '{}'...", file_extension);
					return count_lines_in_file(file_path);
				}
			}
		}
	}
	0
}

fn analyze_path_recursive(
	path: &PathBuf,
	ignored_dirs: &HashSet<String>,
	file_types: &HashSet<String>
) {
	if path.is_dir() {
		let dir_path = path;
		if let Some(dir_name_os_str) = dir_path.file_name() {
			if let Some(dir_name) = dir_name_os_str.to_str() {
				if ignored_dirs.contains(&dir_name.to_lowercase()) {
					// println!("Ignoring directory '{}'...", dir_name);
					return;
				}
			}
		}
		let Ok(paths) = fs::read_dir(dir_path) else {
			eprintln!("{} could not be read as a directory.", dir_path.display());
			return;
		};

		for dir_entry_fallible in paths {
			let Ok(dir_entry) = dir_entry_fallible else {
				eprintln!(
					"Could not use dir_entry: {}",
					dir_entry_fallible.unwrap_err()
				);
				continue;
			};
			let path = dir_entry.path();
			analyze_path_recursive(&path, ignored_dirs, file_types);
		}
	} else if path.is_file() {
		let file_path = path;
		if file_types.is_empty() {
			analyze_file(file_path);
		} else if let Some(file_extension_os_str) = file_path.extension() {
			if let Some(file_extension) = file_extension_os_str.to_str() {
				if file_types.contains(&file_extension.to_lowercase()) {
					// println!("Ignoring file extension '{}'...", file_extension);
					analyze_file(file_path);
				}
			}
		}
	}
}

fn count_lines_in_file(file_path: &PathBuf) -> u128 {
	let Ok(file_str) = fs::read_to_string(file_path) else {
		eprintln!("{:?} could not be read as a file to a string.", file_path);
		return 0;
	};
	u128::from(code_analysis::count_valid_code_lines(&file_str))
}

fn analyze_file(file_path: &PathBuf) {
	let Ok(file_str) = fs::read_to_string(file_path) else {
		eprintln!("{:?} could not be read as a file to a string.", file_path);
		return;
	};

	let name = file_path.file_name().unwrap(); // should be valid because we read it as a file
	let lines_of_code = code_analysis::count_valid_code_lines(&file_str);
	println!("{name:?} has {lines_of_code} lines of code.");
}
