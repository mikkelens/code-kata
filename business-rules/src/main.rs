use std::{
	collections::{BTreeMap, BTreeSet},
	sync::Arc
};

mod data;

#[allow(clippy::wildcard_imports)]
use data::{io::*, types::*};

const EXIT_CHAR: char = 'E';
const UNRECOGNIZED_COMMAND_STR: &str =
	"You typed an unrecognized command. Try using the letters in '[]' above.";
fn main() {
	'program_loop: loop {
		const PURCHASE_DATA_STR: &str = "P";
		const RULE_DATA_STR: &str = "R";
		const QUERY_DATA_STR: &str = "Q";

		println!("What do you want to do?");
		println!(" - [{}] Modify purchase data", PURCHASE_DATA_STR);
		println!(" - [{}] Modify rule data", RULE_DATA_STR);
		println!(" - [{}] Query database", QUERY_DATA_STR);
		println!(" - [{}] Exit", EXIT_CHAR);

		let action_fn = 'input_loop: loop {
			let parsed_decision: Option<fn()> = match get_reply().to_uppercase() {
				s if s.contains(PURCHASE_DATA_STR) => Some(purchase_modify_decision),
				s if s.contains(RULE_DATA_STR) => Some(rule_modify_decision),
				s if s.contains(QUERY_DATA_STR) => Some(query_database),
				s if s.contains(EXIT_CHAR) => break 'program_loop,
				_ => None
			};
			if let Some(action_fn) = parsed_decision {
				break 'input_loop action_fn;
			}
			print_unrecognized_command();
		};
		println!(); // space before
		action_fn();
		println!(); // space after
	}
}
fn print_unrecognized_command() {
	println!("{}", UNRECOGNIZED_COMMAND_STR);
}

const ALL_STR: &str = "A";
const INDIVIDUAL_STR: &str = "I";
const ORDER_STR: &str = "O";

const PRINT_STR: &str = "P";

const ADD_STR: &str = "A";
const MODIFY_STR: &str = "M";
const DELETE_STR: &str = "D";
fn purchase_modify_decision() {
	println!("What do you want to modify in purchase data?");
	println!(" - [{}] Add a new purchase entry", ADD_STR);
	println!(" - [{}] Modify an existing purchase entry", MODIFY_STR);
	println!(" - [{}] Delete an existing purchase entry", DELETE_STR);
	println!(" - [{}] Print information about the data", PRINT_STR);
	println!(" - [{}] Exit", EXIT_CHAR);
	let action_fn = 'input_loop: loop {
		let parsed_decision: Option<fn()> = match get_reply().to_uppercase() {
			s if s.contains(ADD_STR) => Some(add_purchase),
			s if s.contains(MODIFY_STR) => Some(modify_purchase),
			s if s.contains(DELETE_STR) => Some(delete_purchase),
			s if s.contains(PRINT_STR) => Some(print_purchase_data),
			s if s.contains(EXIT_CHAR) => return,
			_ => None
		};
		if let Some(action_fn) = parsed_decision {
			break 'input_loop action_fn;
		}
		print_unrecognized_command();
	};
	println!();
	action_fn();
}
fn rule_modify_decision() {
	println!("What do you want to modify in rule data?");
	println!(" - [{}] Add a new rule entry", ADD_STR);
	println!(" - [{}] Modify an existing rule entry", MODIFY_STR);
	println!(" - [{}] Delete an existing rule entry", DELETE_STR);
	println!(" - [{}] Print information about the data", PRINT_STR);
	println!(" - [{}] Exit", EXIT_CHAR);
	let action_fn = 'input_loop: loop {
		let parsed_decision: Option<fn()> = match get_reply().to_uppercase() {
			s if s.contains(ADD_STR) => Some(add_rule),
			s if s.contains(MODIFY_STR) => unimplemented!(),
			s if s.contains(DELETE_STR) => Some(delete_rule),
			s if s.contains(PRINT_STR) => Some(print_rule_data),
			s if s.contains(EXIT_CHAR) => return,
			_ => None
		};
		if let Some(action_fn) = parsed_decision {
			break 'input_loop action_fn;
		}
		print_unrecognized_command();
	};
	println!();
	action_fn();
}

fn print_purchase_data() {
	println!("What purchase data do you want to print out?");
	println!(" - [{}] All of it", ALL_STR);
	println!(" - [{}] An order of it", ORDER_STR);
	println!(" - [{}] One of them", INDIVIDUAL_STR);
	println!(" - [{}] Exit", EXIT_CHAR);
	let action_fn = 'input_loop: loop {
		let parsed_decision: Option<fn()> = match get_reply().to_uppercase() {
			s if s.contains(ALL_STR) => Some(print_purchase_data_all),
			s if s.contains(ORDER_STR) => Some(print_purchase_data_order),
			s if s.contains(INDIVIDUAL_STR) => Some(print_purchase_data_individual),
			s if s.contains(EXIT_CHAR) => return,
			_ => None
		};
		if let Some(action_fn) = parsed_decision {
			break 'input_loop action_fn;
		}
		print_unrecognized_command();
	};
	println!();
	action_fn();
}
fn print_rule_data() {
	println!("What rule data do you want to print out?");
	println!(" - [{}] All of it", ALL_STR);
	println!(" - [{}] One of them", INDIVIDUAL_STR);
	println!(" - [{}] Exit", EXIT_CHAR);
	let action_fn = 'input_loop: loop {
		let parsed_decision: Option<fn()> = match get_reply().to_uppercase() {
			s if s.contains(ALL_STR) => Some(print_rule_data_all),
			s if s.contains(INDIVIDUAL_STR) => Some(print_rule_data_individual),
			s if s.contains(EXIT_CHAR) => return,
			_ => None
		};
		if let Some(action_fn) = parsed_decision {
			break 'input_loop action_fn;
		}
		print_unrecognized_command();
	};
	println!();
	action_fn();
}

fn query_database() {
	const PROCESSING_STR: &str = "P";
	println!("What do you want to use the database for?");
	println!(" - [{}] Print processing information", PROCESSING_STR);

	let action_fn = 'input_loop: loop {
		let parsed_decision: Option<fn()> = match get_reply().to_uppercase() {
			s if s.contains(PROCESSING_STR) => Some(print_processing_information),
			s if s.contains(EXIT_CHAR) => return,
			_ => None
		};
		if let Some(action_fn) = parsed_decision {
			break 'input_loop action_fn;
		}
		print_unrecognized_command();
	};
	println!();
	action_fn();
	println!();
}
fn print_processing_information() {
	println!("How much processing information do you want to print out?");
	println!(" - [{}] All purchases", ALL_STR);
	println!(" - [{}] Order of purchases", ORDER_STR);
	println!(" - [{}] Individual purchase", INDIVIDUAL_STR);
	println!(" - [{}] Exit", EXIT_CHAR);

	let action_fn = 'input_loop: loop {
		let parsed_decision: Option<fn()> = match get_reply().to_uppercase() {
			s if s.contains(ALL_STR) => Some(print_processing_all),
			s if s.contains(ORDER_STR) => Some(print_processing_order),
			s if s.contains(INDIVIDUAL_STR) => Some(print_processing_individual),
			s if s.contains(EXIT_CHAR) => return,
			_ => None
		};
		if let Some(action_fn) = parsed_decision {
			break 'input_loop action_fn;
		}
		print_unrecognized_command();
	};
	println!();
	action_fn();
}

fn add_purchase() {
	let new_purchase = Purchase {
		title:       Arc::from(prompt_question("Provide a title to the purchase.")),
		identifiers: request_identifiers_answer()
	};
	let mut all_purchases = load_purchases();
	if all_purchases.insert(new_purchase) {
		save_purchases(all_purchases);
		println!("\nSaved item into dataset.");
	} else {
		println!("\nThis exact item already exists in the dataset, skipping saving.");
	}
}
fn add_rule() {
	let rule = Rule {
		title:          Arc::from(prompt_question("What should the title of this rule be?")),
		process_action: Arc::from(prompt_question(
			"What should happen when this rule is triggered?"
		)),
		trigger:        request_rule_trigger_answer()
	};
	let mut rules = load_rules();

	if rules.insert(rule) {
		save_rules(rules);
		println!("\nSaved rule into dataset.");
	} else {
		println!("\nThis exact rule already exists in the dataset, skipping saving.");
	}
}
fn modify_purchase() {
	const TITLE_STR: &str = "T";
	const IDENTIFIER_STR: &str = "I";
	println!("In order to modify a purchase we must first find it.");
	let mut all_purchases = load_purchases();
	if let Some(purchase) = quick_find_purchase(all_purchases.clone().iter()) {
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
			let new_purchase = modifying_fn(purchase.clone());
			if get_yes_no_answer("Are you satisfied with the purchase?") {
				break 'modify_loop;
			}

			let insert_successful = all_purchases.insert(new_purchase);
			if !insert_successful
				|| get_yes_no_answer(
					"Data already contained this exact value. Do you still want to delete the old \
					 value?"
				) {
				let remove_succesful = all_purchases.remove(purchase);
				if !remove_succesful {
					unreachable!("Could not remove purchase we just found?");
				}
			}
		}
	}
}
fn modify_purchase_title(mut purchase: Purchase) -> Purchase {
	//
	todo!()
}
fn modify_purchase_identifiers(mut purchase: Purchase) -> Purchase {
	//
	todo!()
}
fn delete_purchase() {
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
fn delete_rule() {
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

fn print_purchase_data_individual() {
	let purchases = load_purchases();
	let possible_purchase = quick_find_purchase(purchases.iter());
	println!();
	if let Some(purchase) = possible_purchase {
		println!("Purchase:\n{}", purchase);
	} else {
		println!("No purchase with the provided specifications could be found.");
	}
}
fn print_rule_data_individual() {
	let rules = load_rules();
	let possible_rule = quick_find_rule(rules.iter());
	println!();
	if let Some(rule) = possible_rule {
		println!("Purchase:\n{}", rule);
	} else {
		println!("No rule with the provided specifications could be found.");
	}
}
fn print_purchase_data_order() {
	let order = get_order();
	println!();
	if order.purchases.0.is_empty() {
		println!("No purchases in order to print.");
	} else {
		for (index, (purchase, amount)) in order.purchases.0.iter().enumerate() {
			println!("Purchase no. {} (x{}):\n{}", index + 1, amount, purchase);
			println!();
		}
	}
}
fn print_purchase_data_all() {
	let all_purchases = load_purchases();
	for (index, purchase) in all_purchases.iter().enumerate() {
		println!("Purchase no. {}:\n{}", index + 1, purchase);
		println!();
	}
}
fn print_rule_data_all() {
	let all_rules = load_rules();
	for (index, rule) in all_rules.iter().enumerate() {
		println!("Rule no. {}:\n{}", index + 1, rule);
		println!();
	}
}

fn print_processing_individual() {
	let rules = load_rules();
	if rules.is_empty() {
		println!("There are currently no rules to trigger any processes.");
	} else {
		let purchases = load_purchases();
		let possible_purchase = quick_find_purchase(purchases.iter());
		println!();
		if let Some(purchase) = possible_purchase {
			let processing_steps = purchase.get_processing_steps(&rules);
			if processing_steps.is_empty() {
				println!("This purchase does trigger any processing rules.");
			} else {
				println!(
					"The processing steps for this purchase are the following:\n - {}",
					processing_steps
						.iter()
						.map(AsRef::as_ref)
						.collect::<Vec<_>>()
						.join("\n - ")
				);
			}
		} else {
			println!("No item with the provided specifications could be found.");
		}
	}
}
fn print_processing_order() {
	let order = get_order();
	println!(); // post-user-entry spacing
	if order.purchases.0.is_empty() {
		println!("No purchases in order to print.");
	} else {
		let rules = load_rules();
		if rules.is_empty() {
			println!("There are currently no rules to trigger any processes.");
		} else {
			let rules = &rules;
			for (index, (purchase, amount)) in order.purchases.0.iter().enumerate() {
				println!("Purchase no. {} (x{}):\n{}", index + 1, amount, purchase);
				print_processing_steps(purchase, rules);
				println!(); // extra spacing between each
			}
		}
	}
}

fn get_order() -> Order {
	let mut order = Order {
		purchases: PurchaseCollection(BTreeMap::new())
	};
	let all_purchases = load_purchases();
	'purchase_add_loop: loop {
		println!(
			"--- Purchase {} ---",
			order.purchases.0.values().sum::<usize>()
		);
		let new_purchase = 'purchase_find_loop: loop {
			if let Some(purchase) = quick_find_purchase(all_purchases.iter()) {
				break 'purchase_find_loop purchase;
			}
			if !get_yes_no_answer("Failed to find valid purchase. Do you want to try again?") {
				break 'purchase_add_loop;
			}
		};
		order
			.purchases
			.0
			.entry(new_purchase.clone())
			.and_modify(|count| *count += 1)
			.or_insert(1);
		println!("Purchase added to order.");
		if !get_yes_no_answer("Do you want to add another purchase to this order?") {
			break 'purchase_add_loop;
		}
	}
	order
}
fn print_processing_all() {
	let all_purchases = load_purchases();
	let rules = load_rules();
	if rules.is_empty() {
		println!("There are currently no rules to trigger any processes.");
	} else {
		let rules = &rules;
		for (index, purchase) in all_purchases.iter().enumerate() {
			println!("Purchase no. {}:\n{}", index + 1, purchase);
			print_processing_steps(purchase, rules);
			println!(); // extra spacing
		}
	}
}

fn print_processing_steps(purchase: &Purchase, rules: &BTreeSet<Rule>) {
	let processing_steps = purchase.get_processing_steps(rules);
	if processing_steps.is_empty() {
		println!("This purchase does trigger any processing rules.");
	} else {
		println!(
			"The processing steps for this purchase are the following:\n - {}",
			processing_steps
				.iter()
				.map(AsRef::as_ref)
				.collect::<Vec<_>>()
				.join("\n - ")
		);
	}
}

fn request_identifiers_answer() -> IdentifierCollection {
	println!("Please provide some tags (separated by semicolon).");
	let mut all_identifiers = IdentifierCollection::default();
	'add_loop: loop {
		// always start by adding
		let add_reply = get_reply();
		add_str_as_identifier(add_reply, &mut all_identifiers);
		'review_loop: loop {
			println!(
				"Identifiers: [{}]",
				all_identifiers
					.0
					.iter()
					.map(|i| i.0.as_ref())
					.collect::<Vec<_>>()
					.join(", ")
			);
			if get_yes_no_answer("Are you satisfied with the identifiers?") {
				break 'add_loop; // finish and return identifiers
			}
			println!("Do you want to add [A] or delete [D] modifiers? ([C] to cancel)");
			'operation_loop: loop {
				let operation_reply = get_reply();
				match operation_reply.to_lowercase() {
					s if s.contains('a') => {
						println!("What do you want to add? (Still separated by semicolon)");
						let add_reply = get_reply();
						add_str_as_identifier(add_reply, &mut all_identifiers);
						continue 'review_loop;
					},
					s if s.contains('d') => {
						println!("What do you want to delete? (Still separated by semicolon)");
						let delete_reply = get_reply();
						remove_str_as_identifier(delete_reply, &mut all_identifiers);
						continue 'review_loop; // modify complete
					},
					_ => {
						println!("You must use one of the key letters above to signal intent.");
						continue 'operation_loop;
					}
				}
			}
		}
	}
	all_identifiers
}

fn add_str_as_identifier(s: impl AsRef<str>, all_identifiers: &mut IdentifierCollection) {
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
fn remove_str_as_identifier(s: impl AsRef<str>, all_identifiers: &mut IdentifierCollection) {
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

fn request_rule_trigger_answer() -> RuleTrigger {
	println!("Select the type of trigger for this rule:");
	println!(" - [Never] trigger");
	println!(" - [Always] trigger");
	println!(" - Trigger on [Title] match");
	println!(" - Trigger on [Identifier] match");
	println!(" - Trigger on a [Combination] of other rules");
	println!(" - Trigger when another is [Not] triggered");
	'trigger_parse: loop {
		let reply = get_reply();
		break 'trigger_parse match reply.to_lowercase() {
			s if s.contains("never") => RuleTrigger::Never,
			s if s.contains("always") => RuleTrigger::Always,
			s if s.contains("title") => RuleTrigger::Title {
				name: Arc::from(prompt_question(
					"What should the title of the purchase be for this rule to trigger?"
				))
			},
			s if s.contains("identifier") => {
				let identifiers = request_identifiers_answer();
				let condition = match identifiers.0.len() {
					1 => {
						println!("Condition of single identifier is set to 'Any' by default.");
						IdentifierCondition::Any
					},
					_ => 'condition_parse: loop {
						println!("Select the condition to trigger this Identifier rule:");
						println!(" - [None] of the identifiers can be present");
						println!(" - [Any] of the identifiers have to be present");
						println!(" - [All] of the identifiers have to be present");
						let reply = get_reply();
						break 'condition_parse match reply.to_lowercase() {
							s if s.contains("none") => IdentifierCondition::None,
							s if s.contains("any") => IdentifierCondition::Any,
							s if s.contains("all") => IdentifierCondition::All,
							s => {
								println!("'{}' was not recognized as one of the options.", s);
								println!("Try again.");
								continue 'condition_parse;
							}
						};
					}
				};
				RuleTrigger::Identifier {
					identifiers,
					condition
				}
			},
			s if s.contains("combination") => {
				println!("In order to make a combination of rule triggers,");
				println!("you must provide two different triggers.");
				println!("Are you sure you want to proceed?");
				RuleTrigger::Combination {
					a:         {
						println!("--- RULE TRIGGER A ---");
						Box::new(request_rule_trigger_answer())
					},
					b:         {
						println!("--- RULE TRIGGER A ---");
						Box::new(request_rule_trigger_answer())
					},
					condition: {
						'condition_parse: loop {
							println!("Select the combinational trigger of this rule:");
							println!(" - [None] of the triggers need to be active");
							println!(" - [ExactlyOne] of the triggers has to be active");
							println!(" - [Either] one of the triggers has to be active");
							println!(" - [Both] of the triggers have to be cative");
							let reply = get_reply();
							break 'condition_parse match reply.to_lowercase() {
								s if s.contains("none") => CombinationCondition::None,
								s if s.contains("exactly") => CombinationCondition::ExactlyOne,
								s if s.contains("either") => CombinationCondition::Either,
								s if s.contains("both") => CombinationCondition::Both,
								s => {
									println!("'{}' was not recognized as one of the options.", s);
									println!("Try again.");
									continue 'condition_parse;
								}
							};
						}
					}
				}
			},
			s if s.contains("not") => todo!(),
			s => {
				println!("'{}' was not recognized as one of the options.", s);
				println!("Try again.");
				continue 'trigger_parse;
			}
		};
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
				if tag_recognized {
					println!("Tag was not found in the collection of matches.");
					println!("Try again.");
					continue 'tag_specify_loop;
				}
				found_matches.retain(|found_match| found_match.has_identifier(&replied_tag));
				break 'tag_specify_loop;
			}
			if found_matches.len() > 1 {
				println!("Multiple purchases share provided tag(s) in the dataset.");
				println!("Tags present for purchases with specified name and tags:");
				continue 'tag_narrow_loop;
			}
			break 'tag_narrow_loop;
		}
		Some(found_matches.remove(0))
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

const PURCHASE_DATA_PATH: &str =
	r"C:\Users\mikke\Desktop\repos\rust\code-kata\business-rules\src\all_purchases.json";
fn load_purchases() -> BTreeSet<Purchase> { load_set(PURCHASE_DATA_PATH) }
fn save_purchases(purchases: BTreeSet<Purchase>) {
	save_overwrite_path(purchases, PURCHASE_DATA_PATH);
}
const RULE_DATA_PATH: &str =
	r"C:\Users\mikke\Desktop\repos\rust\code-kata\business-rules\src\all_rules.json";
fn load_rules() -> BTreeSet<Rule> { load_set(RULE_DATA_PATH) }
fn save_rules(rules: BTreeSet<Rule>) { save_overwrite_path(rules, RULE_DATA_PATH); }
