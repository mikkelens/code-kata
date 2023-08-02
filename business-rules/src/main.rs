use std::{collections::BTreeSet, sync::Arc};

use lazy_static::lazy_static;

mod data;
mod interaction;
mod printing;

#[allow(clippy::wildcard_imports)]
use data::{io::*, types::*};
#[allow(clippy::wildcard_imports)]
use interaction::*;
#[allow(clippy::wildcard_imports)]
use printing::*;

fn main() {
	'program_loop: loop {
		lazy_static! {
			static ref DECISION: Decision<fn()> = Decision {
				possible_choices: vec![
					(
						("P", "Modify purchase data").into(),
						purchase_modify_decision as fn()
					)
						.into(),
					(
						("R", "Modify rule data").into(),
						rule_modify_decision as fn()
					)
						.into(),
					(("Q", "Query database").into(), query_database as fn()).into()
				],
				cancel_answer: Answer::exit_answer(),
				..Default::default()
			};
		}

		if let Some(action) = DECISION.run_prompt() {
			action();
		} else {
			break 'program_loop;
		}
	}
}

fn purchase_modify_decision() {
	lazy_static! {
		static ref DECISION: Decision<fn()> = Decision {
			prompt: "What do you want to modify in purchase data?",
			possible_choices: vec![
				(
					("A", "Add a new purchase entry").into(),
					add_a_purchase as fn()
				)
					.into(),
				(
					("M", "Modify an existing purchase entry").into(),
					modify_a_purchase as fn()
				)
					.into(),
				(
					("D", "Delete an existing purchase entry").into(),
					delete_a_purchase as fn()
				)
					.into(),
				(
					("P", "Print information about the data").into(),
					print_purchase_data as fn()
				)
					.into(),
			],
			..Default::default()
		};
	}
	if let Some(action) = DECISION.run_prompt() {
		action();
	}
}
fn rule_modify_decision() {
	lazy_static! {
		static ref DECISION: Decision<fn()> = Decision {
			prompt: "What do you want to modify in rule data?",
			possible_choices: vec![
				(("A", "Add a new rule entry").into(), add_a_rule as fn()).into(),
				(
					("M", "Modify an existing rule entry").into(),
					modify_a_rule as fn()
				)
					.into(),
				(
					("D", "Delete an existing rule entry").into(),
					delete_a_rule as fn()
				)
					.into(),
				(
					("P", "Print information about the data").into(),
					print_rule_data as fn()
				)
					.into(),
			],
			..Default::default()
		};
	}
	if let Some(action) = DECISION.run_prompt() {
		action();
	}
}

fn print_purchase_data() {
	lazy_static! {
		static ref DECISION: Decision<fn()> = Decision {
			prompt: "What purchase data do you want to print out?",
			possible_choices: vec![
				(("A", "All of them").into(), print_purchase_data_all as fn()).into(),
				(
					("O", "An order of them").into(),
					print_purchase_data_order as fn()
				)
					.into(),
				(
					("I", "Individual purchase").into(),
					print_purchase_data_individual as fn()
				)
					.into(),
			],
			..Default::default()
		};
	}
	if let Some(action) = DECISION.run_prompt() {
		action();
	}
}
fn print_rule_data() {
	lazy_static! {
		static ref DECISION: Decision<fn()> = Decision {
			prompt: "What rule data do you want to print out?",
			possible_choices: vec![
				(("A", "All of them").into(), print_rule_data_all as fn()).into(),
				(
					("I", "Individual rule").into(),
					print_rule_data_individual as fn()
				)
					.into(),
			],
			..Default::default()
		};
	}
	if let Some(action) = DECISION.run_prompt() {
		action();
	}
}

fn query_database() {
	lazy_static! {
		static ref DECISION: Decision<fn()> = Decision {
			prompt: "What do you want to use the database for?",
			possible_choices: vec![(
				("P", "Print processing information").into(),
				print_processing_decision as fn()
			)
				.into()],
			..Default::default()
		};
	}
	if let Some(action) = DECISION.run_prompt() {
		action();
	}
}
fn print_processing_decision() {
	lazy_static! {
		static ref DECISION: Decision<fn()> = Decision {
			prompt: "How much processing information do you want to print out?",
			possible_choices: vec![
				(("A", "All purchases").into(), print_processing_all as fn()).into(),
				(
					("O", "Order of purchases").into(),
					print_processing_order as fn()
				)
					.into(),
				(
					("I", "Individual purchase").into(),
					print_processing_individual as fn()
				)
					.into(),
			],
			..Default::default()
		};
	}
	if let Some(action) = DECISION.run_prompt() {
		action();
	}
}

fn add_a_purchase() {
	let new_purchase = Purchase {
		title:       Arc::from(prompt_question("Provide a title to the purchase.")),
		identifiers: IdentifierCollection::prompt_creation()
	};
	let mut all_purchases = load_purchases();
	if all_purchases.insert(new_purchase) {
		save_purchases(all_purchases);
		println!("\nSaved item into dataset.");
	} else {
		println!("\nThis exact item already exists in the dataset, skipping saving.");
	}
}
fn add_a_rule() {
	let rule = Rule::prompt_creation();
	let mut rules = load_rules();
	if rules.insert(rule) {
		save_rules(rules);
		println!("\nSaved rule into dataset.");
	} else {
		println!("\nThis exact rule already exists in the dataset, skipping saving.");
	}
}
fn modify_a_purchase() {
	type FnType = fn(Purchase) -> Purchase;
	lazy_static! {
		static ref DECISION: Decision<FnType> = Decision {
			prompt: "What do you want to change about this purchase?",
			possible_choices: vec![
				(
					("T", "Modify title").into(),
					modify_purchase_title as FnType
				)
					.into(),
				(
					("I", "Modify identifiers").into(),
					modify_purchase_identifiers as FnType
				)
					.into(),
			],
			..Default::default()
		};
	}
	let mut all_purchases = load_purchases();
	println!("In order to modify a purchase we must first find it.");
	if let Some(purchase) = quick_find_purchase(all_purchases.clone().iter()) {
		let mut purchase_modified = purchase.clone();

		'modify_loop: loop {
			if let Some(modifying_fn) = DECISION.run_prompt() {
				println!();
				purchase_modified = modifying_fn(purchase_modified);
				if get_yes_no_answer("Are you satisfied with the changes made to the purchase?") {
					break 'modify_loop;
				}
			}
		}
		let insert_successful = all_purchases.insert(purchase_modified);
		if insert_successful
			|| get_yes_no_answer(
				"Data already contained this exact value. Do you still want to delete the old \
				 value?"
			) {
			let remove_succesful = all_purchases.remove(purchase);
			if !remove_succesful {
				unreachable!("Could not remove purchase we just found?");
			}
		}
		save_purchases(all_purchases);
	}
}
fn modify_purchase_title(mut purchase: Purchase) -> Purchase {
	purchase.title = Arc::from(prompt_question("What would you like the new title to be?"));
	purchase
}
fn modify_purchase_identifiers(mut purchase: Purchase) -> Purchase {
	modify_identifiercollection_directly(&mut purchase.identifiers);
	purchase
}
fn modify_a_rule() {
	type FnType = fn(Rule) -> Rule;
	lazy_static! {
		static ref DECISION: Decision<FnType> = Decision {
			prompt: "What do you want to change about this rule?",
			possible_choices: vec![
				(("T", "Modify title").into(), modify_rule_title as FnType).into(),
				(
					("P", "Modify process action").into(),
					modify_rule_process_action as FnType
				)
					.into(),
				(
					("R", "Modify rule trigger").into(),
					modify_rule_trigger as FnType
				)
					.into(),
			],
			..Default::default()
		};
	}
	let mut all_rules = load_rules();
	println!("In order to modify a rule we must first find it.");
	if let Some(rule) = quick_find_rule(all_rules.clone().iter()) {
		let mut rule_modified = rule.clone();
		'modify_loop: loop {
			if let Some(modifying_fn) = DECISION.run_prompt() {
				println!();
				rule_modified = modifying_fn(rule_modified);
				if get_yes_no_answer("Are you satisfied with the changes made to the rule?") {
					break 'modify_loop;
				}
			}
		}
		let insert_successful = all_rules.insert(rule_modified);
		if insert_successful
			|| get_yes_no_answer(
				"Data already contained this exact value. Do you still want to delete the old \
				 value?"
			) {
			let remove_succesful = all_rules.remove(rule);
			if !remove_succesful {
				unreachable!("Could not remove rule we just found?");
			}
		}
		save_rules(all_rules);
	}
}
fn modify_rule_title(mut rule: Rule) -> Rule {
	rule.title = Arc::from(prompt_question("What would you like the new title to be?"));
	rule
}
fn modify_rule_process_action(mut rule: Rule) -> Rule {
	rule.title = Arc::from(prompt_question(
		"What would you like the new process action to be?"
	));
	rule
}
fn modify_rule_trigger(mut rule: Rule) -> Rule {
	rule.trigger = RuleTrigger::prompt_creation();
	rule
}
fn delete_a_purchase() {
	println!("In order to delete a purchase we must first find it.");
	let mut all_purchases = load_purchases();
	if let Some(purchase) = quick_find_purchase(all_purchases.clone().iter()) {
		let confirmation_question =
			format!("Are you sure you want to delete...\n{:?}\n...?", purchase);
		if get_yes_no_answer(confirmation_question) {
			all_purchases.remove(purchase);
			save_purchases(all_purchases);
			println!("\nPurchase was removed from dataset.");
		} else {
			println!("\nPurchase was kept in dataset.");
		}
	} else {
		println!("\nNo purchase with the provided specifications could be found.");
	}
}
fn delete_a_rule() {
	println!("In order to delete a rule we must first find it.");
	let rules = load_rules();
	let mut updated_rules = rules.clone();
	if let Some(rule) = quick_find_rule(rules.iter()) {
		let confirmation_question = format!("Are you sure you want to delete...\n{:?}\n...?", rule);
		if get_yes_no_answer(confirmation_question) {
			updated_rules.remove(rule);
			save_rules(updated_rules);
			println!("\nRule was removed from dataset.");
		} else {
			println!("\nRule was kept in dataset.");
		}
	} else {
		println!("\nNo rule with the provided specifications could be found.");
	}
}

fn modify_identifiercollection_directly(all_identifiers: &mut IdentifierCollection) {
	type FnType = fn(&mut IdentifierCollection, String);
	lazy_static! {
		static ref DECISION: Decision<FnType> = Decision {
			prompt: "What do you want to change about these modifiers?",
			possible_choices: vec![
				(("A", "Add identifiers").into(), add_from_str as FnType).into(),
				(
					("D", "Delete identifiers").into(),
					remove_from_str as FnType
				)
					.into(),
			],
			..Default::default()
		};
	}
	if let Some(modifying_fn) = DECISION.run_prompt() {
		let identifier_reply = get_reply();
		modifying_fn(all_identifiers, identifier_reply);
	}
}

fn add_from_str(all_identifiers: &mut IdentifierCollection, s: impl AsRef<str>) {
	let identifiers_to_add = s.as_ref().split(';').map(str::trim).collect::<Vec<_>>();
	for identifier in identifiers_to_add {
		if identifier.is_empty() {
			println!("'' is an empty identifier and is skipped.");
		} else if !all_identifiers.0.insert(identifier.into()) {
			println!(
				"'{}' is already an identifier for this entry, skipping addition...",
				identifier
			);
		}
	}
}
fn remove_from_str(all_identifiers: &mut IdentifierCollection, s: impl AsRef<str>) {
	let identifiers_for_removal = s
		.as_ref()
		.split(';')
		.map(|i_str| i_str.trim().into())
		.collect::<Vec<Identifier>>();
	for identifier in identifiers_for_removal {
		if !all_identifiers.0.remove(&identifier) {
			println!(
				"'{}' is not an identifier for this entry, skipping removal...",
				identifier.0
			);
		}
	}
}

fn quick_find_purchase<'a>(data: impl Iterator<Item = &'a Purchase>) -> Option<&'a Purchase> {
	let title = prompt_question("What is the title of the purchase?");
	let mut found_matches: Vec<&Purchase> = data
		.filter(|purchase| purchase.title_matches(&title))
		.collect();
	if found_matches.is_empty() {
		None
	} else if found_matches.len() == 1 {
		Some(found_matches.remove(0))
	} else {
		println!("Purchases with the same name were found in the dataset.");
		println!("Tags present for purchases with this name:",);

		'tag_narrow_loop: loop {
			let unique_tags: BTreeSet<&Identifier> = found_matches
				.iter()
				.flat_map(|purchase| purchase.get_all_ídentifiers()) // iter of vecs to vec
				.collect(); // unique
			println!(
				"[{}] (unordered)",
				unique_tags
					.into_iter()
					.map(|i| i.0.as_ref())
					.collect::<Vec<_>>()
					.join(", ")
			);
			println!("Choose a tag to narrow the search by.");
			'tag_specify_loop: loop {
				let replied_tag: Identifier = get_reply().into();
				let tag_recognized = found_matches
					.iter()
					.any(|found_match| found_match.has_identifier(&replied_tag));
				if !tag_recognized {
					println!("Tag was not found in the collection of matches.");
					println!("Try again.");
					continue 'tag_specify_loop;
				}
				found_matches.retain(|found_match| found_match.has_identifier(&replied_tag));
				break 'tag_specify_loop;
			}
			if found_matches.is_empty() {
				break 'tag_narrow_loop None;
			}
			if found_matches.len() == 1 {
				break 'tag_narrow_loop Some(found_matches.remove(0));
			}
			println!("Multiple purchases share provided tag(s) in the dataset.");
			println!("Tags present for purchases with specified name and tags:");
		}
	}
}
fn quick_find_rule<'a>(data: impl Iterator<Item = &'a Rule>) -> Option<&'a Rule> {
	let title = prompt_question("What is the title of the rule?");
	let mut found_matches: Vec<&Rule> = data
		.filter(|purchase| purchase.title.as_ref() == title)
		.collect();
	if found_matches.is_empty() {
		None
	} else if found_matches.len() == 1 {
		Some(found_matches.remove(0))
	} else {
		println!("Rules with the same name were found in the dataset.");
		for (index, rule) in found_matches.iter().enumerate() {
			println!("Rule {} process_action: {}", index, rule.process_action);
		}
		println!("Choose a rule using its index (number to the left).");
		let selected_match = 'index_request_loop: loop {
			let reply = get_reply();
			let Ok(unsigned_int_reply) = reply.trim().parse::<usize>() else {
				println!("Reply was not an unsigned integer.");
				println!("Try again.");
				continue 'index_request_loop;
			};
			if unsigned_int_reply >= found_matches.len() {
				println!("Reply index was not inside the range of the found matches.");
				println!("Try again.");
				continue 'index_request_loop;
			}
			break 'index_request_loop found_matches.remove(unsigned_int_reply);
		};
		Some(selected_match)
	}
}
