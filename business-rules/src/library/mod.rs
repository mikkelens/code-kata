pub(crate) mod decisions;
pub(crate) mod io;
pub(crate) mod printing;
pub(crate) mod searching;
pub(crate) mod types;

use std::{collections::BTreeMap, sync::Arc};

#[allow(clippy::wildcard_imports)]
use crate::*;

pub(crate) trait UserCreated {
	fn prompt_creation() -> Self;
}
impl UserCreated for IdentifierCollection {
	fn prompt_creation() -> Self {
		println!("Please provide some tags (separated by semicolon).");
		let mut all_identifiers = IdentifierCollection::default();
		// always start by adding
		let add_reply = get_reply();
		add_from_str(&mut all_identifiers, add_reply);
		'review_modify_loop: loop {
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
				break 'review_modify_loop;
			}
			modify_identifiercollection_directly(&mut all_identifiers);
		}
		all_identifiers
	}
}

impl UserCreated for Purchase {
	fn prompt_creation() -> Self {
		Purchase {
			title:       Arc::from(prompt_question("Provide a title to the purchase.")),
			identifiers: IdentifierCollection::prompt_creation()
		}
	}
}
impl UserCreated for Rule {
	fn prompt_creation() -> Self {
		Rule {
			title:          Arc::from(prompt_question("What should the title of this rule be?")),
			process_action: Arc::from(prompt_question(
				"What should happen when this rule is triggered?"
			)),
			trigger:        RuleTrigger::prompt_creation()
		}
	}
}
impl UserCreated for Order {
	fn prompt_creation() -> Self {
		let purchases = {
			let all_purchases = Purchase::load_from_disk();
			let mut purchases = PurchaseCollection(BTreeMap::new());
			'purchase_add_loop: loop {
				println!("--- Purchase {} ---", purchases.0.values().sum::<usize>());
				let new_purchase = 'purchase_find_loop: loop {
					if let Some(purchase) = quick_find_purchase(all_purchases.iter()) {
						break 'purchase_find_loop purchase;
					}
					if !get_yes_no_answer(
						"Failed to find valid purchase. Do you want to try again?"
					) {
						break 'purchase_add_loop;
					}
				};
				purchases
					.0
					.entry(new_purchase.clone())
					.and_modify(|count| *count += 1)
					.or_insert(1);
				println!("Purchase added to order.");
				if !get_yes_no_answer("Do you want to add another purchase to this order?") {
					break 'purchase_add_loop;
				}
			}
			purchases
		};
		Order { purchases }
	}
}
impl UserCreated for RuleTrigger {
	fn prompt_creation() -> Self {
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
					let identifiers = IdentifierCollection::prompt_creation();
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
							Box::new(Self::prompt_creation())
						},
						b:         {
							println!("--- RULE TRIGGER A ---");
							Box::new(Self::prompt_creation())
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
										println!(
											"'{}' was not recognized as one of the options.",
											s
										);
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
}

// could add blackbox-like tests here
