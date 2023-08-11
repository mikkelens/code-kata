use std::fmt::Display;

use lazy_static::lazy_static;

#[allow(clippy::wildcard_imports)]
use crate::library::{decisions::*, io::*, printing::*, searching::*, types::*, user_creation::*};
use crate::{
	try_modify_purchase_identifiers, try_modify_purchase_title, try_modify_rule_process_action,
	try_modify_rule_title, try_modify_rule_trigger
};

pub(crate) mod decisions;
pub(crate) mod io;
pub(crate) mod printing;
pub(crate) mod searching;
pub(crate) mod types;
pub(crate) mod user_creation;

pub type PathDataFn = fn(&ApplicationData);

pub(crate) trait DatabaseEntry:
	NeatPrintable + Display + PathFindable + Saved + TryUserCreate + for<'a> Searchable<'a>
{
	fn print_decision(data: &ApplicationData);
	fn entry_action_decision(data: &ApplicationData) {
		let decision: Decision<PathDataFn> = Decision {
			prompt: "What do you want to modify in purchase data?".into(),
			possible_choices: vec![
				(
					("A", "Add a new purchase entry").into(),
					Self::add_entry as PathDataFn
				)
					.into(),
				(
					("M", "Modify an existing purchase entry").into(),
					Self::modify_entry as PathDataFn
				)
					.into(),
				(
					("D", "Delete an existing purchase entry").into(),
					Self::delete_entry as PathDataFn
				)
					.into(),
				(
					("P", "Print information about the data").into(),
					Purchase::print_decision as PathDataFn
				)
					.into(),
			],
			..Default::default()
		};
		if let Some(action) = decision.run_prompt() {
			action(data);
		} else {
			println!("Canceled entry action, returning...");
		}
	}
	fn add_entry(data: &ApplicationData) {
		if let Some(new) = Self::try_prompt_creation() {
			let mut all = Self::load_from_disk(Self::get_path(data));
			println!();
			if all.insert(new) {
				Self::save_to_disk(Self::get_path(data), all);
				println!("Saved {} into its dataset.", Self::type_name_pretty());
			} else {
				println!(
					"This exact {} already exists in its dataset, skipping saving.",
					Self::type_name_pretty()
				);
			}
		} else {
			println!("Failed to create entry.");
		}
	}
	fn try_ask_modify_type() -> Option<fn(Self) -> Option<Self>>;
	fn modify_entry(data: &ApplicationData) {
		let mut all = Self::load_from_disk(Self::get_path(data));
		println!(
			"In order to modify a {} we must first find it.",
			Self::type_name_pretty().to_lowercase()
		);
		if let Some(found) = Self::quick_find(all.clone().iter()) {
			println!(
				"Modifying {}:\n{}",
				Self::type_name_pretty().to_lowercase(),
				found
			);
			let mut entry_modified = found.clone();
			'modify_loop: loop {
				if let Some(modifying_fn) = Self::try_ask_modify_type() {
					println!();
					if let Some(modified_entry) = modifying_fn(entry_modified.clone()) {
						entry_modified = modified_entry;
						if get_yes_no_answer(format!(
							"Are you satisfied with the changes made to the {}?\n{}",
							Self::type_name_pretty().to_lowercase(),
							entry_modified
						)) {
							break 'modify_loop;
						}
					} else {
						println!("{} modification canceled.", Self::type_name_pretty());
						if *found == entry_modified {
							println!("No modifications were made, returning...");
							return;
						} else if !get_yes_no_answer("Do you want to make more modifications?") {
							break 'modify_loop;
						}
					}
				} else {
					println!("Canceled, returning...");
					return;
				}
			}
			// todo: account for what happens when you modify something and then unmodify it
			if all.insert(entry_modified)
				|| get_yes_no_answer(
					"Database already contained this exact new value. Do you still want to delete \
					 the old value?"
				) {
				let remove_succesful = all.remove(found);
				assert!(
					remove_succesful,
					"Could not remove entry that we just found?"
				);
			}
			Self::save_to_disk(Self::get_path(data), all);
		} else {
			println!("Failed to find entry using this name.");
		}
	}
	fn delete_entry(data: &ApplicationData) {
		println!(
			"In order to delete a {} we must first find it.",
			Self::type_name_pretty()
		);
		let mut all = Self::load_from_disk(Self::get_path(data));
		if let Some(found) = Self::quick_find(all.clone().iter()) {
			let confirmation_question =
				format!("Are you sure you want to delete...\n{}\n...?", found);
			if get_yes_no_answer(confirmation_question) {
				all.remove(found);
				Self::save_to_disk(Self::get_path(data), all);
				println!("\n{} was removed from dataset.", Self::type_name_pretty());
			} else {
				println!("\n{} was kept in dataset.", Self::type_name_pretty());
			}
		} else {
			println!(
				"\nNo {} with the provided specifications could be found.",
				Self::type_name_pretty()
			);
		}
	}
	fn print_data_individual(data: &ApplicationData) {
		let all = Self::load_from_disk(Self::get_path(data));
		let possible_item = Self::quick_find(all.iter());
		println!();
		if let Some(item) = possible_item {
			item.print();
		} else {
			println!(
				"No {} with the provided specifications could be found.",
				Self::type_name_pretty()
			);
		}
	}
	fn print_data_all(data: &ApplicationData) { Self::load_from_disk(Self::get_path(data)).print() }
}

impl DatabaseEntry for Purchase {
	fn print_decision(data: &ApplicationData) {
		lazy_static! {
			static ref DECISION: Decision<PathDataFn> = Decision {
				prompt: "What purchase data do you want to print out?".into(),
				possible_choices: vec![
					(
						("A", "All of them").into(),
						Purchase::print_data_all as PathDataFn
					)
						.into(),
					(
						("O", "An order of them").into(),
						print_purchase_data_order as PathDataFn
					)
						.into(),
					(
						("I", "Individual purchase").into(),
						Purchase::print_data_individual as PathDataFn
					)
						.into(),
				],
				..Default::default()
			};
		}
		if let Some(action) = DECISION.run_prompt() {
			action(data);
		}
	}

	fn try_ask_modify_type() -> Option<fn(Self) -> Option<Self>> {
		type FnType = fn(Purchase) -> Option<Purchase>;
		lazy_static! {
			static ref DECISION: Decision<FnType> = Decision {
				prompt: "What do you want to change about this purchase?".into(),
				possible_choices: vec![
					(
						("T", "Modify title").into(),
						try_modify_purchase_title as FnType
					)
						.into(),
					(
						("I", "Modify identifiers").into(),
						try_modify_purchase_identifiers as FnType
					)
						.into(),
				],
				..Default::default()
			};
		}
		DECISION.run_prompt()
	}
}
impl DatabaseEntry for Rule {
	fn print_decision(data: &ApplicationData) {
		lazy_static! {
			static ref DECISION: Decision<PathDataFn> = Decision {
				prompt: "What rule data do you want to print out?".into(),
				possible_choices: vec![
					(
						("A", "All of them").into(),
						Rule::print_data_all as PathDataFn
					)
						.into(),
					(
						("I", "Individual rule").into(),
						Rule::print_data_individual as PathDataFn
					)
						.into(),
				],
				..Default::default()
			};
		}
		if let Some(action) = DECISION.run_prompt() {
			action(data);
		}
	}

	fn try_ask_modify_type() -> Option<fn(Self) -> Option<Self>> {
		type FnType = fn(Rule) -> Option<Rule>;
		lazy_static! {
			static ref DECISION: Decision<FnType> = Decision {
				prompt: "What do you want to change about this rule?".into(),
				possible_choices: vec![
					(
						("T", "Modify title").into(),
						try_modify_rule_title as FnType
					)
						.into(),
					(
						("P", "Modify process action").into(),
						try_modify_rule_process_action as FnType
					)
						.into(),
					(
						("R", "Modify rule trigger").into(),
						try_modify_rule_trigger as FnType
					)
						.into(),
				],
				..Default::default()
			};
		}
		DECISION.run_prompt()
	}
}
