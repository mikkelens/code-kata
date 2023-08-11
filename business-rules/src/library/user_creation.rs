use std::{collections::BTreeMap, sync::Arc};

#[allow(clippy::wildcard_imports)]
use crate::*;

pub(crate) trait TryUserCreate
where
	Self: Sized
{
	fn try_prompt_creation() -> Option<Self>;
}
impl TryUserCreate for IdentifierCollection {
	fn try_prompt_creation() -> Option<Self> {
		println!("Please provide some tags (separated by semicolon).");
		let mut all_identifiers = IdentifierCollection::default();
		// always start by adding
		let add_reply = try_get_reply()?;
		all_identifiers = add_from_str(all_identifiers, add_reply);
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
			all_identifiers = try_modify_identifiercollection(all_identifiers)?;
		}
		Some(all_identifiers)
	}
}

impl TryUserCreate for Purchase {
	fn try_prompt_creation() -> Option<Self> {
		Some(Purchase {
			title:       Arc::from(try_prompt_question("Provide a title to the purchase.")?),
			identifiers: IdentifierCollection::try_prompt_creation()?
		})
	}
}
impl TryUserCreate for Rule {
	fn try_prompt_creation() -> Option<Self> {
		Some(Rule {
			title:          Arc::from(prompt_question("What should the title of this rule be?")),
			process_action: Arc::from(prompt_question(
				"What should happen when this rule is triggered?"
			)),
			trigger:        RuleTrigger::try_prompt_creation()?
		})
	}
}

pub(crate) trait UserSelected {
	fn prompt_data_selection(data: &ApplicationData) -> Self;
}
impl UserSelected for Order {
	fn prompt_data_selection(data: &ApplicationData) -> Self {
		let purchases = {
			let all_purchases = Purchase::load_from_disk(Purchase::get_path(data));
			let mut purchases = PurchaseCollection(BTreeMap::new());
			'purchase_add_loop: loop {
				println!("--- Purchase {} ---", purchases.0.values().sum::<usize>());
				let new_purchase = 'purchase_find_loop: loop {
					if let Some(purchase) = Purchase::quick_find(all_purchases.iter()) {
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
#[derive(Clone)]
enum RuleTriggerSurface {
	Never,
	Always,
	Title,
	Identifier,
	Combination,
	Not
}
impl RuleTriggerSurface {
	fn try_complete(self) -> Option<RuleTrigger> {
		Some(match self {
			RuleTriggerSurface::Never => RuleTrigger::Never,
			RuleTriggerSurface::Always => RuleTrigger::Always,
			RuleTriggerSurface::Title => RuleTrigger::Title {
				name: Arc::from(prompt_question(
					"What should the title of the purchase be for this rule to trigger?"
				))
			},
			RuleTriggerSurface::Identifier => {
				let identifiers = IdentifierCollection::try_prompt_creation()?;
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
			RuleTriggerSurface::Combination => {
				println!("In order to make a combination of rule triggers,");
				println!("you must provide two different triggers.");
				println!("Are you sure you want to proceed?");
				RuleTrigger::Combination {
					a:         {
						println!("--- RULE TRIGGER A ---");
						Box::new(RuleTrigger::try_prompt_creation()?)
					},
					b:         {
						println!("--- RULE TRIGGER A ---");
						Box::new(RuleTrigger::try_prompt_creation()?)
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
			RuleTriggerSurface::Not => todo!()
		})
	}
}
impl TryUserCreate for RuleTrigger {
	fn try_prompt_creation() -> Option<Self> {
		lazy_static! {
			static ref SURFACE_DECISION: Decision<RuleTriggerSurface> = Decision {
				prompt: "Select the type of trigger for this rule:".into(),
				possible_choices: vec![
					(("N", "Never trigger").into(), RuleTriggerSurface::Never).into(),
					(("A", "Always trigger").into(), RuleTriggerSurface::Always).into(),
					(
						("T", "Trigger on title match").into(),
						RuleTriggerSurface::Title
					)
						.into(),
					(
						("I", "Trigger on identifier match").into(),
						RuleTriggerSurface::Identifier
					)
						.into(),
					(
						("2", "Trigger on a combination of two other rules").into(),
						RuleTriggerSurface::Combination
					)
						.into(),
					(
						("!", "Trigger when another rule is not triggered").into(),
						RuleTriggerSurface::Not
					)
						.into()
				],
				..Default::default()
			};
		}
		SURFACE_DECISION.run_prompt()?.try_complete()
	}
}

// could add blackbox-like tests here
