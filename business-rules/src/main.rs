use std::sync::Arc;

mod library;

use lazy_static::lazy_static;
#[allow(clippy::wildcard_imports)]
use library::{decisions::*, io::*, printing::*, searching::*, types::*, user_creation::*, *};

fn main() {
	'program_loop: loop {
		lazy_static! {
			static ref DECISION: Decision<fn()> = Decision {
				possible_choices: vec![
					(
						("P", "Modify purchase data").into(),
						Purchase::entry_action_decision as fn()
					)
						.into(),
					(
						("R", "Modify rule data").into(),
						Rule::entry_action_decision as fn()
					)
						.into(),
					(("Q", "Query database").into(), query_database as fn()).into()
				],
				cancel_answer: Answer::exit_answer(),
				..Default::default()
			};
		}

		println!("----- MAIN MENU -----");

		if let Some(action) = DECISION.run_prompt() {
			action();
		} else {
			break 'program_loop;
		}
	}
}

fn query_database() {
	lazy_static! {
		static ref DECISION: Decision<fn()> = Decision {
			prompt: "What do you want to use the database for?".into(),
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
			prompt: "How much processing information do you want to print out?".into(),
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
fn try_modify_purchase_title(mut purchase: Purchase) -> Option<Purchase> {
	purchase.title = Arc::from(try_prompt_question(
		"What would you like the new title to be?"
	)?);
	Some(purchase)
}
fn try_modify_purchase_identifiers(mut purchase: Purchase) -> Option<Purchase> {
	purchase.identifiers = try_modify_identifiercollection(purchase.identifiers)?;
	Some(purchase)
}
fn try_modify_rule_title(mut rule: Rule) -> Option<Rule> {
	rule.title = Arc::from(try_prompt_question(
		"What would you like the new title to be?"
	)?);
	Some(rule)
}
fn try_modify_rule_process_action(mut rule: Rule) -> Option<Rule> {
	rule.title = Arc::from(try_prompt_question(
		"What would you like the new process action to be?"
	)?);
	Some(rule)
}
fn try_modify_rule_trigger(mut rule: Rule) -> Option<Rule> {
	rule.trigger = RuleTrigger::try_prompt_creation()?;
	Some(rule)
}
fn try_modify_identifiercollection(
	mut all_identifiers: IdentifierCollection
) -> Option<IdentifierCollection> {
	type FnType = fn(IdentifierCollection, String) -> IdentifierCollection;
	lazy_static! {
		static ref DECISION: Decision<FnType> = Decision {
			prompt: "What about do you want to change about the identifiers?".into(),
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
		let identifier_reply = try_prompt_question("Which identifiers (separated by semicolon)?")?;
		all_identifiers = modifying_fn(all_identifiers, identifier_reply);
		Some(all_identifiers)
	} else {
		None
	}
}
fn add_from_str(
	mut all_identifiers: IdentifierCollection,
	s: impl AsRef<str>
) -> IdentifierCollection {
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
	all_identifiers
}
fn remove_from_str(
	mut all_identifiers: IdentifierCollection,
	s: impl AsRef<str>
) -> IdentifierCollection {
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
	all_identifiers
}
