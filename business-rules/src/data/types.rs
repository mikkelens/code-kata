use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

pub type Identifier = String;
pub type IdentifierCollection = BTreeSet<Identifier>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Purchase {
	pub title:       String,
	pub identifiers: IdentifierCollection
}
impl Purchase {
	pub fn get_processing_steps(&self, rules: &BTreeSet<Rule>) -> Vec<Identifier> {
		rules
			.iter()
			.filter(|rule| rule.trigger.triggered_by(self))
			.map(|triggered_rule| triggered_rule.process_action.clone())
			.collect()
	}

	pub fn title_matches(&self, other_title: &str) -> bool {
		self.title.to_lowercase().trim() == other_title.to_lowercase().trim()
	}

	pub fn has_identifier(&self, identifier: &str) -> bool { self.identifiers.contains(identifier) }

	pub fn get_all_ídentifiers(&self) -> Vec<&str> {
		self.identifiers
			.iter()
			.map(|s| s.as_ref())
			.collect::<Vec<_>>()
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
		name: String
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
				IdentifierCondition::Any => {
					identifiers.iter().any(|i| purchase.identifiers.contains(i))
				},
				IdentifierCondition::All => {
					identifiers.iter().all(|i| purchase.identifiers.contains(i))
				},
				IdentifierCondition::None => {
					!identifiers.iter().any(|i| purchase.identifiers.contains(i))
				},
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
	pub title:          String,
	pub process_action: String, // process
	pub trigger:        RuleTrigger
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
					identifiers: ["physical".into()].into(),
					condition:   IdentifierCondition::Any
				}
			},
			Rule {
				title:          "royalty gets their duplicate slip".into(),
				process_action: DUPLICATE_SLIP.into(),
				trigger:        RuleTrigger::Identifier {
					identifiers: ["book".into()].into(),
					condition:   IdentifierCondition::Any
				}
			},
			Rule {
				title:          "memberships get activated".into(),
				process_action: ACTIVATE_MEMBERSHIP.into(),
				trigger:        RuleTrigger::Identifier {
					identifiers: ["membership".into()].into(),
					condition:   IdentifierCondition::Any
				}
			},
			Rule {
				title:          "membership upgrade get applied".into(),
				process_action: APPLY_UPGRADE.into(),
				trigger:        RuleTrigger::Identifier {
					identifiers: ["membership upgrade".into()].into(),
					condition:   IdentifierCondition::Any
				}
			},
			Rule {
				title:          "owner is informed of memberships and upgrades".into(),
				process_action: EMAIL_OWNER.into(),
				trigger:        RuleTrigger::Identifier {
					identifiers: ["membership".into(), "upgrade".into()].into(),
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
					identifiers: ["physical".into(), "book".into()].into(),
					condition:   IdentifierCondition::Any
				}
			}
		]);
	}

	#[test]
	fn book_print() {
		let expectation: Vec<String> = vec![
			GENERATE_SLIP.into(),
			DUPLICATE_SLIP.into(),
			GENERATE_COMMISION.into(),
		];
		let purchase = Purchase {
			title:       "1984".into(),
			identifiers: ["physical".into(), "book".into()].into()
		};
		let steps = purchase.get_processing_steps(&RULES_SAMPLE);
		assert!(expectation.iter().all(|item| steps.contains(item)));
	}
	#[test]
	fn ski_mp4_print() {
		let expectation: Vec<String> = vec![FIRST_AID_VIDEO.into()];
		let purchase = Purchase {
			title:       "Learning to Ski".into(),
			identifiers: ["video".into()].into()
		};
		let steps = purchase.get_processing_steps(&RULES_SAMPLE);
		assert!(expectation.iter().all(|item| steps.contains(item)));
	}
	#[test]
	fn gym_membership_print() {
		let expectation: Vec<String> = vec![ACTIVATE_MEMBERSHIP.into(), EMAIL_OWNER.into()];
		let purchase = Purchase {
			title:       "Fitness World 3 month discount trial".into(),
			identifiers: ["membership".into()].into()
		};
		let steps = purchase.get_processing_steps(&RULES_SAMPLE);
		assert!(expectation.iter().all(|item| steps.contains(item)));
	}
}
