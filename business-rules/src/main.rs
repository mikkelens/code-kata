use std::{
	any::type_name,
	collections::{BTreeSet, HashSet},
	fs,
	io::{stdin, stdout, Write}
};

use business_rules::*;

fn main() {
	'program_loop: loop {
		const ADD_PURCHASE_STR: &str = "AP";
		const DELETE_PURCHASE_STR: &str = "DP";
		const ADD_RULE_STR: &str = "AR";
		const DELETE_RULE_STR: &str = "DR";
		const PRINT_INDIVIDUAL_STR: &str = "PI";
		const PRINT_ALL_STR: &str = "PA";
		const EXIT_CHAR: char = 'E';
		println!("What do you want to do?");
		println!(" - [{}] Add purchase entry", ADD_PURCHASE_STR);
		println!(" - [{}] Delete purchase entry", DELETE_PURCHASE_STR);
		println!(" - [{}] Add rule", ADD_RULE_STR);
		println!(" - [{}] Delete rule", DELETE_RULE_STR);
		println!(
			" - [{}] Print processing steps for individual purchase",
			PRINT_INDIVIDUAL_STR
		);
		println!(
			" - [{}] Print processing steps for *all* purchases",
			PRINT_ALL_STR
		);
		println!(" - [{}] Exit", EXIT_CHAR);

		'input_loop: loop {
			let user_str = get_reply(); // blocking IO

			match user_str.to_uppercase() {
				s if s.contains(ADD_PURCHASE_STR) => {
					println!();
					let new_purchase = Purchase {
						title:       prompt_question("Provide a title to the purchase."),
						identifiers: request_identifiers_answer()
					};
					let mut purchases = load_purchases();
					if purchases.insert(new_purchase) {
						save_purchases(purchases);
						println!("\nSaved item into dataset.");
					} else {
						println!(
							"\nThis exact item already exists in the dataset, skipping saving."
						);
					}
				},
				s if s.contains(DELETE_PURCHASE_STR) => {
					println!();
					println!("In order to delete a purchase we must first find it.");
					let purchases = load_purchases();
					let mut updated_purchases = purchases.clone();
					if let Some(purchase) = quick_find_purchase(purchases.into_iter().collect()) {
						let confirmation_question =
							format!("Are you sure you want to delete...\n{:?}\n...?", purchase);
						if get_yes_no_answer(&confirmation_question) {
							updated_purchases.remove(&purchase);
							save_purchases(updated_purchases);
							println!("\nPurchase was removed from dataset.")
						} else {
							println!("\nPurchase was kept in dataset.");
						}
					} else {
						println!("\nNo purchase with the provided specifications could be found.");
					}
				},
				s if s.contains(ADD_RULE_STR) => {
					println!();
					let rule = Rule {
						title:          prompt_question("What should the title of this rule be?"),
						process_action: prompt_question(
							"What should happen when this rule is triggered?"
						),
						trigger:        request_rule_trigger_answer()
					};
					let mut rules = load_rules();

					if rules.insert(rule) {
						save_rules(rules);
						println!("\nSaved rule into dataset.");
					} else {
						println!(
							"\nThis exact rule already exists in the dataset, skipping saving."
						);
					}
				},
				s if s.contains(DELETE_RULE_STR) => {
					println!();
					println!("In order to delete a rule we must first find it.");
					let rules = load_rules();
					let mut updated_rules = rules.clone();
					if let Some(rule) = quick_find_rule(rules.into_iter().collect()) {
						let confirmation_question =
							format!("Are you sure you want to delete...\n{:?}\n...?", rule);
						if get_yes_no_answer(&confirmation_question) {
							updated_rules.remove(&rule);
							save_rules(updated_rules);
							println!("\nRule was removed from dataset.")
						} else {
							println!("\nRule was kept in dataset.");
						}
					} else {
						println!("\nNo rule with the provided specifications could be found.");
					}
				},
				s if s.contains(PRINT_INDIVIDUAL_STR) => {
					println!();
					let rules = load_rules();
					if rules.is_empty() {
						println!("There are currently no rules to trigger any processes.");
					} else {
						let purchases = load_purchases();
						let possible_purchase =
							quick_find_purchase(purchases.into_iter().collect());
						println!();
						if let Some(purchase) = possible_purchase {
							let processing_steps = purchase.get_processing_steps(&rules);
							if processing_steps.is_empty() {
								println!("This purchase does trigger any processing rules.");
							} else {
								println!(
									"The processing steps for this purchase are the following:\n \
									 - {}",
									processing_steps.join("\n - ")
								);
							}
						} else {
							println!("No item with the provided specifications could be found.");
						}
					}
				},
				s if s.contains(PRINT_ALL_STR) => {
					println!();
					let all_purchases = load_purchases();
					let rules = load_rules();
					if rules.is_empty() {
						println!("There are currently no rules to trigger any processes.");
					} else {
						for (index, purchase) in all_purchases.iter().enumerate() {
							println!("PURCHASE {}:\n{:?}", index, purchase);
							let processing_steps = purchase.get_processing_steps(&rules);
							if processing_steps.is_empty() {
								println!("This purchase does trigger any processing rules.");
							} else {
								println!(
									"The processing steps for this purchase are the following:\n \
									 - {}",
									processing_steps.join("\n - ")
								);
							}
							println!() // extra spacing
						}
					}
				},
				s if s.contains(EXIT_CHAR) => break 'program_loop,
				_ => {
					println!(
						"You typed an unrecognized command. Try using the letters in '[]' above."
					);
					continue 'input_loop;
				}
			}
			break 'input_loop;
		}
		println!(); // completed command operation, space for effect
	}
}

fn request_identifiers_answer() -> TagCollection {
	println!("Please provide some tags (separated by semicolon).");
	let mut identifiers = TagCollection::default();
	'modify_loop: loop {
		// always start by adding
		let reply = get_reply();
		let identifiers_to_add = reply.split(';').map(|s| s.trim()).collect::<Vec<_>>();
		for identifier in identifiers_to_add {
			if !identifiers.insert(identifier.into()) {
				println!(
					"'{}' is already an identifier for this entry, skipping addition...",
					identifier
				);
			}
		}
		'review_loop: loop {
			println!(
				"Identifiers: ['{}']",
				identifiers
					.clone()
					.into_iter()
					.collect::<Vec<_>>()
					.join("', '")
			);
			if get_yes_no_answer("Are you satisfied with the identifiers?") {
				break 'modify_loop; // finish and return identifiers
			} else {
				println!("Do you want to add [A] or delete [D] modifiers? ('C' to cancel)");
				'operation_loop: loop {
					let reply = get_reply();
					match reply.to_lowercase() {
						s if s.contains('a') => {
							println!("What do you want to add? (Still separated by semicolon)");
							continue 'modify_loop; // reuse start
						},
						s if s.contains('d') => {
							println!("What do you want to delete? (Still separated by semicolon)");
							let reply = get_reply();
							let identifiers_for_removal =
								reply.split(';').map(|s| s.trim()).collect::<Vec<&str>>();
							for identifier in identifiers_for_removal {
								if !identifiers.remove(identifier) {
									println!(
										"'{}' is not an identifier for this entry, skipping \
										 removal...",
										identifier
									);
								}
							}
							continue 'review_loop; // modify complete
						},
						_ => {
							println!("You must use one of the key letters above to signal intent.");
							continue 'operation_loop; // try input again
						}
					}
				}
			}
		}
	}
	identifiers
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
				name: prompt_question(
					"What should the title of the purchase be for this rule to trigger?"
				)
			},
			s if s.contains("identifier") => {
				let identifiers = request_identifiers_answer();
				let condition = match identifiers.len() {
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

fn quick_find_purchase(data: Vec<Purchase>) -> Option<Purchase> {
	let title = prompt_question("What is the title of the purchase?");
	let mut found_matches: Vec<Purchase> = data
		.into_iter()
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
			let unique_tags: HashSet<String> = found_matches
				.iter()
				.map(|found_match| found_match.get_all_tags().into_iter().collect())
				.collect();
			println!(
				"[{}] (unordered)",
				unique_tags.into_iter().collect::<Vec<_>>().join(", ")
			);
			println!("Choose a tag to narrow the search by.");
			'tag_specify_loop: loop {
				let replied_tag = get_reply();
				let tag_recognized = found_matches
					.iter()
					.any(|found_match| found_match.has_identifier(replied_tag.as_str()));
				if tag_recognized {
					println!("Tag was not found in the collection of matches.");
					println!("Try again.");
					continue 'tag_specify_loop;
				}
				found_matches
					.retain(|found_match| found_match.has_identifier(replied_tag.as_str()));
				break 'tag_specify_loop;
			}
			if found_matches.len() > 1 {
				println!("Multiple purchases share provided tag(s) in the dataset.");
				println!("Tags present for purchases with specified name and tags:");
				continue 'tag_narrow_loop;
			} else {
				break 'tag_narrow_loop;
			}
		}
		Some(found_matches.remove(0))
	}
}
fn quick_find_rule(data: Vec<Rule>) -> Option<Rule> {
	let title = prompt_question("What is the title of the rule?");
	let mut found_matches: Vec<Rule> = data
		.into_iter()
		.filter(|purchase| purchase.title == title)
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

fn load_set<T: serde::de::DeserializeOwned + Ord>(path: &str) -> BTreeSet<T> {
	'reading: loop {
		let data_string: String = fs::read_to_string(path).expect("could not read from file");
		if data_string.is_empty() {
			break 'reading BTreeSet::new();
		} else if let Ok(set) = serde_json::from_str(data_string.as_str()) {
			break 'reading set;
		} else {
			println!(
				"Could not parse data as {} from path '{}'",
				type_name::<BTreeSet<Purchase>>(),
				path
			);
			println!("Press enter to retry.");
			let _ = read_line();
		}
	}
}
fn save_set_overwrite<T: serde::Serialize>(set: BTreeSet<T>, path: &str) {
	let data_string = serde_json::to_string_pretty(&set).expect("should always be able to parse");
	'write: loop {
		if fs::write(path, data_string.clone()).is_err() {
			println!(
				"Could not write data as {} to path '{}'.",
				type_name::<BTreeSet<Purchase>>(),
				path
			);
			println!("Press enter key to retry.");
			let _ = read_line();
		} else {
			break 'write;
		}
	}
}
const PURCHASE_DATA_PATH: &str = "F:/Git/Rust/code-kata/business-rules/src/all_items.json";
fn load_purchases() -> BTreeSet<Purchase> { load_set(PURCHASE_DATA_PATH) }
fn save_purchases(purchases: BTreeSet<Purchase>) {
	save_set_overwrite(purchases, PURCHASE_DATA_PATH)
}
const RULE_DATA_PATH: &str = "F:/Git/Rust/code-kata/business-rules/src/all_rules.json";
fn load_rules() -> BTreeSet<Rule> { load_set(RULE_DATA_PATH) }
fn save_rules(rules: BTreeSet<Rule>) { save_set_overwrite(rules, RULE_DATA_PATH) }

fn get_yes_no_answer(question: &str) -> bool {
	println!("{} (Y/N)", question);
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
		println!("You need to answer with a yes [Y] or no [N].")
	}
}
fn prompt_question(question: &str) -> String {
	println!("{question}");
	get_reply()
}
fn get_reply() -> String {
	print!("> ");
	// flush enables us to write without a newline and have it display pre-input
	stdout().flush().expect("flush failed"); // possibly breaks everything in certain terminal environments
	read_line().trim().to_string()
}
fn read_line() -> String {
	let mut buffer = String::new();
	stdin().read_line(&mut buffer).expect("unable to read line");
	buffer
}
