use std::{
	collections::{BTreeMap, BTreeSet},
	fmt::Display,
	sync::Arc
};

use serde::{Deserialize, Serialize};

use super::io::{load_purchases, prompt_question};
use crate::{
	add_from_str,
	data::io::{get_reply, get_yes_no_answer},
	modify_identifiercollection_directly, quick_find_purchase
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Identifier(pub Arc<str>);
impl<T: AsRef<str>> From<T> for Identifier {
	fn from(value: T) -> Self { Identifier(Arc::from(value.as_ref())) }
}
impl Display for Identifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "'{}'", self.0) }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct IdentifierCollection(pub BTreeSet<Identifier>);
impl From<&[Identifier]> for IdentifierCollection {
	fn from(value: &[Identifier]) -> Self { IdentifierCollection(value.iter().cloned().collect()) }
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Purchase {
	pub title:       Arc<str>,
	pub identifiers: IdentifierCollection
}
impl Purchase {
	pub fn get_processing_steps(&self, rules: &BTreeSet<Rule>) -> Vec<Arc<str>> {
		rules
			.iter()
			.filter(|rule| rule.trigger.triggered_by(self))
			.map(|triggered_rule| triggered_rule.process_action.clone())
			.collect()
	}

	pub fn title_matches(&self, other_title: &str) -> bool {
		self.title.to_lowercase().trim() == other_title.to_lowercase().trim()
	}

	pub fn has_identifier(&self, identifier: &Identifier) -> bool {
		self.identifiers.0.contains(identifier)
	}

	pub fn get_all_ídentifiers(&self) -> Vec<&Identifier> { self.identifiers.0.iter().collect() }
}
impl Display for Purchase {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"title: '{}',\nidentifiers: [{}]",
			self.title.as_ref(),
			self.identifiers
				.0
				.iter()
				.map(|identifier| format!("{}", identifier))
				.collect::<Vec<_>>()
				.join(", ")
		)
	}
}

#[derive(Debug)]
pub struct PurchaseCollection(pub BTreeMap<Purchase, usize>);
#[derive(Debug)]
pub struct Order {
	pub purchases: PurchaseCollection
}
impl UserCreated for Order {
	fn prompt_creation() -> Self {
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum IdentifierCondition {
	None,
	Any,
	All
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CombinationCondition {
	None,
	ExactlyOne,
	Either,
	Both
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RuleTrigger {
	Never,
	Always,
	Title {
		name: Arc<str>
	},
	Identifier {
		identifiers: IdentifierCollection,
		condition:   IdentifierCondition
	},
	Combination {
		a:         Box<RuleTrigger>,
		b:         Box<RuleTrigger>,
		condition: CombinationCondition
	},
	Not {
		flipped_rule: Box<RuleTrigger>
	}
}
pub trait UserCreated {
	fn prompt_creation() -> Self;
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
impl RuleTrigger {
	fn triggered_by(&self, purchase: &Purchase) -> bool {
		match self {
			RuleTrigger::Never => false,
			RuleTrigger::Always => true,
			RuleTrigger::Title { name } => purchase.title_matches(name),
			RuleTrigger::Identifier {
				identifiers,
				condition
			} => match condition {
				IdentifierCondition::Any => identifiers
					.0
					.iter()
					.any(|i| purchase.identifiers.0.contains(i)),
				IdentifierCondition::All => identifiers
					.0
					.iter()
					.all(|i| purchase.identifiers.0.contains(i)),
				IdentifierCondition::None => !identifiers
					.0
					.iter()
					.any(|i| purchase.identifiers.0.contains(i))
			},
			RuleTrigger::Combination { a, b, condition } => match condition {
				CombinationCondition::None => {
					!{ a.triggered_by(purchase) || b.triggered_by(purchase) }
				},
				CombinationCondition::ExactlyOne => {
					a.triggered_by(purchase) != b.triggered_by(purchase)
				},
				CombinationCondition::Either => {
					a.triggered_by(purchase) || b.triggered_by(purchase)
				},
				CombinationCondition::Both => a.triggered_by(purchase) && b.triggered_by(purchase)
			},
			RuleTrigger::Not { flipped_rule } => !flipped_rule.triggered_by(purchase)
		}
	}
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Rule {
	pub title:          Arc<str>,
	pub process_action: Arc<str>, // process
	pub trigger:        RuleTrigger
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
impl Display for Rule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"title: '{}',\nprocess_action:{}\nidentifiers: [{:?}]",
			self.title.as_ref(),
			self.process_action.as_ref(),
			self.trigger
		)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::BTreeSet;

	use lazy_static::lazy_static;

	use super::*;

	const GENERATE_SLIP: &str = "generate a packing slip for shipping";
	const DUPLICATE_SLIP: &str = "create a duplicate packing slip for the royalty department";
	const ACTIVATE_MEMBERSHIP: &str = "activate that membership";
	const APPLY_UPGRADE: &str = "apply the upgrade";
	const EMAIL_OWNER: &str = "e-mail the owner and inform them of the activation/upgrade";
	const FIRST_AID_VIDEO: &str =
		"add a free “First Aid” video to the packing slip (the result of a court decision in 1997)";
	const GENERATE_COMMISION: &str = "generate a commission payment to the agent";

	lazy_static! {
		static ref RULES_SAMPLE: BTreeSet<Rule> = BTreeSet::from([
			Rule {
				title:          "physical products generate slips".into(),
				process_action: GENERATE_SLIP.into(),
				trigger:        RuleTrigger::Identifier {
					identifiers: IdentifierCollection(["physical".into()].into()),
					condition:   IdentifierCondition::Any
				}
			},
			Rule {
				title:          "royalty gets their duplicate slip".into(),
				process_action: DUPLICATE_SLIP.into(),
				trigger:        RuleTrigger::Identifier {
					identifiers: IdentifierCollection(["book".into()].into()),
					condition:   IdentifierCondition::Any
				}
			},
			Rule {
				title:          "memberships get activated".into(),
				process_action: ACTIVATE_MEMBERSHIP.into(),
				trigger:        RuleTrigger::Identifier {
					identifiers: IdentifierCollection(["membership".into()].into()),
					condition:   IdentifierCondition::Any
				}
			},
			Rule {
				title:          "membership upgrade get applied".into(),
				process_action: APPLY_UPGRADE.into(),
				trigger:        RuleTrigger::Identifier {
					identifiers: IdentifierCollection(["membership upgrade".into()].into()),
					condition:   IdentifierCondition::Any
				}
			},
			Rule {
				title:          "owner is informed of memberships and upgrades".into(),
				process_action: EMAIL_OWNER.into(),
				trigger:        RuleTrigger::Identifier {
					identifiers: IdentifierCollection(
						["membership".into(), "upgrade".into()].into()
					),
					condition:   IdentifierCondition::Any
				}
			},
			Rule {
				title:          "Learning to Ski first aid video".into(),
				process_action: FIRST_AID_VIDEO.into(),
				trigger:        RuleTrigger::Title {
					name: "Learning to Ski".into()
				}
			},
			Rule {
				title:          "physical products or books generate commission payment".into(),
				process_action: GENERATE_COMMISION.into(),
				trigger:        RuleTrigger::Identifier {
					identifiers: IdentifierCollection(["physical".into(), "book".into()].into()),
					condition:   IdentifierCondition::Any
				}
			}
		]);
	}

	#[test]
	fn book_print() {
		let expectation = vec![
			GENERATE_SLIP.into(),
			DUPLICATE_SLIP.into(),
			GENERATE_COMMISION.into(),
		];
		let purchase = Purchase {
			title:       "1984".into(),
			identifiers: IdentifierCollection(["physical".into(), "book".into()].into())
		};
		let steps = purchase.get_processing_steps(&RULES_SAMPLE);
		assert!(expectation.iter().all(|item| steps.contains(item)));
	}
	#[test]
	fn ski_mp4_print() {
		let expectation = vec![FIRST_AID_VIDEO.into()];
		let purchase = Purchase {
			title:       "Learning to Ski".into(),
			identifiers: IdentifierCollection(["video".into()].into())
		};
		let steps = purchase.get_processing_steps(&RULES_SAMPLE);
		assert!(expectation.iter().all(|item| steps.contains(item)));
	}
	#[test]
	fn gym_membership_print() {
		let expectation = vec![ACTIVATE_MEMBERSHIP.into(), EMAIL_OWNER.into()];
		let purchase = Purchase {
			title:       "Fitness World 3 month discount trial".into(),
			identifiers: IdentifierCollection(["membership".into()].into())
		};
		let steps = purchase.get_processing_steps(&RULES_SAMPLE);
		assert!(expectation.iter().all(|item| steps.contains(item)));
	}
}
