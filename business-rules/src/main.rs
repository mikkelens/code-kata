use std::sync::Arc;

mod library;

use lazy_static::lazy_static;
#[allow(clippy::wildcard_imports)]
use library::{decisions::*, io::*, printing::*, searching::*, types::*, *};

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
	let new_purchase = Purchase::prompt_creation();
	let mut all_purchases = Purchase::load_from_disk();
	if all_purchases.insert(new_purchase) {
		Purchase::save_to_disk(all_purchases);
		println!("\nSaved item into dataset.");
	} else {
		println!("\nThis exact item already exists in the dataset, skipping saving.");
	}
}
fn add_a_rule() {
	let rule = Rule::prompt_creation();
	let mut rules = Rule::load_from_disk();
	if rules.insert(rule) {
		Rule::save_to_disk(rules);
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
	let mut all_purchases = Purchase::load_from_disk();
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
		Purchase::save_to_disk(all_purchases);
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
	let mut all_rules = Rule::load_from_disk();
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
		Rule::save_to_disk(all_rules);
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
	let mut all_purchases = Purchase::load_from_disk();
	if let Some(purchase) = quick_find_purchase(all_purchases.clone().iter()) {
		let confirmation_question =
			format!("Are you sure you want to delete...\n{:?}\n...?", purchase);
		if get_yes_no_answer(confirmation_question) {
			all_purchases.remove(purchase);
			Purchase::save_to_disk(all_purchases);
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
	let rules = Rule::load_from_disk();
	let mut updated_rules = rules.clone();
	if let Some(rule) = quick_find_rule(rules.iter()) {
		let confirmation_question = format!("Are you sure you want to delete...\n{:?}\n...?", rule);
		if get_yes_no_answer(confirmation_question) {
			updated_rules.remove(rule);
			Rule::save_to_disk(updated_rules);
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
