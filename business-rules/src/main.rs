use std::{collections::BTreeSet, sync::Arc};

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
		const QUESTION: Decision = Decision {
			possible_actions: &[
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
			cancel_choice: Answer::exit_answer(),
			..Default::default()
		};

		if !QUESTION.ask_continue() {
			break 'program_loop;
		}
	}
}

fn purchase_modify_decision() {
	const QUESTION: Decision = Decision {
		prompt: "What do you want to modify in purchase data?",
		possible_actions: &[
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
				.into()
		],
		..Default::default()
	};

	QUESTION.ask_continue();
}
fn rule_modify_decision() {
	const QUESTION: Decision = Decision {
		prompt: "What do you want to modify in rule data?",
		possible_actions: &[
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
				.into()
		],
		..Default::default()
	};
	QUESTION.ask_continue();
}

fn print_purchase_data() {
	const QUESTION: Decision = Decision {
		prompt: "What purchase data do you want to print out?",
		possible_actions: &[
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
				.into()
		],
		..Default::default()
	};
	QUESTION.ask_continue();
}
fn print_rule_data() {
	const QUESTION: Decision = Decision {
		prompt: "What rule data do you want to print out?",
		possible_actions: &[
			(("A", "All of them").into(), print_rule_data_all as fn()).into(),
			(
				("I", "Individual rule").into(),
				print_rule_data_individual as fn()
			)
				.into()
		],
		..Default::default()
	};
	QUESTION.ask_continue();
}

fn query_database() {
	const QUESTION: Decision = Decision {
		prompt: "What do you want to use the database for?",
		possible_actions: &[(
			("P", "Print processing information").into(),
			print_processing_decision as fn()
		)
			.into()],
		..Default::default()
	};
	QUESTION.ask_continue();
}
fn print_processing_decision() {
	const QUESTION: Decision = Decision {
		prompt: "How much processing information do you want to print out?",
		possible_actions: &[
			(("A", "All purchases").into(), print_processing_all as fn()).into(),
			(
				("O", "Order of purchases").into(),
				print_processing_order as fn()
			)
				.into(),
			(
				("i", "Individual purchase").into(),
				print_processing_order as fn()
			)
				.into()
		],
		..Default::default()
	};
	QUESTION.ask_continue();
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
	const TITLE_STR: &str = "T";
	const IDENTIFIER_STR: &str = "I";
	let mut all_purchases = load_purchases();
	println!("In order to modify a purchase we must first find it.");
	if let Some(purchase) = quick_find_purchase(all_purchases.clone().iter()) {
		let mut purchase_modified = purchase.clone();

		
		'modify_loop: loop {
			println!("What do you want to change about this purchase?");
			println!(" - [{}] Modify title", TITLE_STR);
			println!(" - [{}] Modify identifiers", IDENTIFIER_STR);
			println!(" - [{}] Exit", EXIT_CHAR);
			let modifying_fn = 'input_loop: loop {
				let parsed_decision: Option<fn(Purchase) -> Purchase> =
					match get_reply().to_uppercase() {
						s if s.contains(TITLE_STR) => Some(modify_purchase_title),
						s if s.contains(IDENTIFIER_STR) => Some(modify_purchase_identifiers),
						s if s.contains(EXIT_CHAR) => return,
						_ => None
					};
				if let Some(action_fn) = parsed_decision {
					break 'input_loop action_fn;
				}
				print_unrecognized_command();
			};
			println!();
			purchase_modified = modifying_fn(purchase_modified);
			if get_yes_no_answer("Are you satisfied with the changes made to the purchase?") {
				break 'modify_loop;
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
	const TITLE_STR: &str = "T";
	const PROCESS_ACTION_STR: &str = "P";
	const RULE_TRIGGER_STR: &str = "R";
	let mut all_rules = load_rules();
	println!("In order to modify a rule we must first find it.");
	if let Some(rule) = quick_find_rule(all_rules.clone().iter()) {
		let mut rule_modified = rule.clone();
		'modify_loop: loop {
			println!("What do you want to change about this rule?");
			println!(" - [{}] Modify title", TITLE_STR);
			println!(" - [{}] Modify process action", PROCESS_ACTION_STR);
			println!(" - [{}] Modify rule trigger", RULE_TRIGGER_STR);
			println!(" - [{}] Exit", EXIT_CHAR);
			let modifying_fn = 'input_loop: loop {
				let parsed_decision: Option<fn(Rule) -> Rule> = match get_reply().to_uppercase() {
					s if s.contains(TITLE_STR) => Some(modify_rule_title),
					s if s.contains(PROCESS_ACTION_STR) => Some(modify_rule_process_action),
					s if s.contains(RULE_TRIGGER_STR) => Some(modify_rule_trigger),
					s if s.contains(EXIT_CHAR) => return,
					_ => None
				};
				if let Some(action_fn) = parsed_decision {
					break 'input_loop action_fn;
				}
				print_unrecognized_command();
			};
			println!();
			rule_modified = modifying_fn(rule_modified);
			if get_yes_no_answer("Are you satisfied with the changes made to the rule?") {
				break 'modify_loop;
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

const CANCEL_STR: &str = "C";

fn modify_identifiercollection_directly(all_identifiers: &mut IdentifierCollection) {
	println!(
		"Do you want to add [{}] or delete [{}] modifiers? ([{}] to cancel)",
		ADD_STR, DELETE_STR, CANCEL_STR
	);
	let modifying_fn: fn(&mut IdentifierCollection, String) = 'input_loop: loop {
		break 'input_loop match get_reply().to_uppercase() {
			s if s.contains(ADD_STR) => {
				println!("What do you want to add? (Still separated by semicolon)");
				add_from_str
			},
			s if s.contains(DELETE_STR) => {
				println!("What do you want to delete? (Still separated by semicolon)");
				remove_from_str
			},
			s if s.contains(CANCEL_STR) => {
				return;
			},
			_ => {
				println!("You must use one of the key letters above to signal intent.");
				continue 'input_loop;
			}
		};
	};
	let identifier_reply = get_reply();
	modifying_fn(all_identifiers, identifier_reply);
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
