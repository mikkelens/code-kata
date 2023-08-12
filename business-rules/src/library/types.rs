use std::{
	collections::{BTreeMap, BTreeSet},
	fmt::Display,
	sync::Arc
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct Identifier(pub(crate) Arc<str>);
impl<T: AsRef<str>> From<T> for Identifier {
	fn from(value: T) -> Self { Identifier(Arc::from(value.as_ref())) }
}
impl Display for Identifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "'{}'", self.0) }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub(crate) struct IdentifierCollection(pub(crate) BTreeSet<Identifier>);
impl From<&[Identifier]> for IdentifierCollection {
	fn from(value: &[Identifier]) -> Self { IdentifierCollection(value.iter().cloned().collect()) }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct Purchase {
	pub(crate) title:       Arc<str>,
	pub(crate) identifiers: IdentifierCollection
}
impl Purchase {
	pub(crate) fn get_processing_steps(&self, rules: &BTreeSet<Rule>) -> Vec<Arc<str>> {
		rules
			.iter()
			.filter(|rule| rule.trigger.triggered_by(self))
			.map(|triggered_rule| triggered_rule.process_action.clone())
			.collect()
	}

	pub(crate) fn has_identifier(&self, identifier: &Identifier) -> bool {
		self.identifiers.0.contains(identifier)
	}

	pub(crate) fn get_all_ídentifiers(&self) -> Vec<&Identifier> {
		self.identifiers.0.iter().collect()
	}
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
pub(crate) struct PurchaseCollection(pub(crate) BTreeMap<Purchase, usize>);
#[derive(Debug)]
pub(crate) struct Order {
	pub(crate) purchases: PurchaseCollection
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum IdentifierCondition {
	None,
	Any,
	All
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum CombinationCondition {
	None,
	ExactlyOne,
	Either,
	Both
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum RuleTrigger {
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

impl RuleTrigger {
	fn triggered_by(&self, purchase: &Purchase) -> bool {
		match self {
			RuleTrigger::Never => false,
			RuleTrigger::Always => true,
			RuleTrigger::Title { name } => {
				purchase.title.to_lowercase().trim() == name.to_lowercase().trim()
			},
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
pub(crate) struct Rule {
	pub(crate) title:          Arc<str>,
	pub(crate) process_action: Arc<str>, // process
	pub(crate) trigger:        RuleTrigger
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

pub trait Named {
	fn name(&self) -> &str;
}
impl Named for Purchase {
	fn name(&self) -> &str { self.title.as_ref() }
}
impl Named for Rule {
	fn name(&self) -> &str { self.title.as_ref() }
}
impl Named for Identifier {
	fn name(&self) -> &str { &self.0 }
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
