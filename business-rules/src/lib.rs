use std::collections::{BTreeSet, HashSet};

use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
// Clone)]
pub type Tags = BTreeSet<String>;
// impl Tags {
// 	fn new() -> Self { Tags(BTreeSet::new()) }
// }
// impl Default for Tags {
// 	fn default() -> Self { Self::new() }
// }
// impl<T> From<T> for Tags
// where
// 	T: AsRef<[String]> // K: AsRef<str>
// {
// 	fn from(value: T) -> Self { value.into() }
// }

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct AltTags(HashSet<String>);
impl PartialOrd for AltTags {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for AltTags {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		let length_ordering = self.0.len().cmp(&self.0.len());
		match length_ordering {
			std::cmp::Ordering::Equal => {
				// unsorted comparison
				let self_tags: Vec<_> = self.0.iter().collect();
				let other_tags: Vec<_> = other.0.iter().collect();
				self_tags.cmp(&other_tags)
			},
			unequal_ord => unequal_ord
		}
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Purchase {
	pub title:   String,
	identifiers: Tags
}
impl Purchase {
	pub fn get_processing_steps(&self) -> Vec<&str> {
		let mut steps: Vec<&str> = vec![];

		if self.has_tag("physical") {
			steps.push(GENERATE_SLIP);
		}
		if self.has_tag("book") {
			steps.push(DUPLICATE_SLIP);
		}
		if self.has_tag("membership") {
			steps.push(ACTIVATE_MEMBERSHIP);
		}
		if self.has_tag("membership_upgrade") {
			steps.push(APPLY_UPGRADE);
		}
		if self.has_any_tags(["membership", "membership_upgrade"].into_iter()) {
			steps.push(EMAIL_OWNER);
		}
		if self.has_tag("video") && self.title == "Learning to Ski" {
			steps.push(ADD_FIRST_AID_VIDEO);
		}
		if self.has_any_tags(["physical", "book"].into_iter()) {
			steps.push(GENERATE_COMMISION);
		}

		steps
	}

	pub fn has_tag(&self, value: impl AsRef<str>) -> bool {
		self.identifiers.contains(value.as_ref())
	}

	pub fn has_any_tags<T: AsRef<str>>(&self, mut values: impl Iterator<Item = T>) -> bool {
		values.any(|value| self.identifiers.contains(value.as_ref()))
	}

	pub fn has_all_tags<T: AsRef<str>>(&self, mut values: impl Iterator<Item = T>) -> bool {
		values.all(|value| self.identifiers.contains(value.as_ref()))
	}
}

const GENERATE_SLIP: &str = "generate a packing slip for shipping";
const DUPLICATE_SLIP: &str = "create a duplicate packing slip for the royalty department";
const ACTIVATE_MEMBERSHIP: &str = "activate that membership";
const APPLY_UPGRADE: &str = "apply the upgrade";
const EMAIL_OWNER: &str = "e-mail the owner and inform them of the activation/upgrade";
const ADD_FIRST_AID_VIDEO: &str =
	"add a free “First Aid” video to the packing slip (the result of a court decision in 1997)";
const GENERATE_COMMISION: &str = "generate a commission payment to the agent";

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn book() {
		let expectation = vec![GENERATE_SLIP, DUPLICATE_SLIP, GENERATE_COMMISION];
		let purchase = Purchase {
			title:       "1984".into(),
			identifiers: ["physical".into(), "book".into()].into()
		};
		assert_eq!(expectation, purchase.get_processing_steps())
	}
	#[test]
	fn ski_mp4() {
		let expectation = vec![ADD_FIRST_AID_VIDEO];
		let purchase = Purchase {
			title:       "Learning to Ski".into(),
			identifiers: ["video".into()].into() // physicality: Physicality::Nonphysical,
		};
		assert_eq!(expectation, purchase.get_processing_steps())
	}
	#[test]
	fn gym_membership() {
		let expectation = vec![ACTIVATE_MEMBERSHIP, EMAIL_OWNER];
		let purchase = Purchase {
			title:       "Fitness World 3 month discount trial".into(),
			identifiers: ["membership".into()].into() // physicality: Physicality::Nonphysical,
		};
		assert_eq!(expectation, purchase.get_processing_steps())
	}
}
