use std::collections::BTreeSet;

use super::{
	io::{prompt_question, try_prompt_question},
	printing::NeatPrintable,
	types::{Purchase, Rule, Titled}
};
use crate::library::{io::get_reply, types::Identifier};

pub trait Searchable<'a>
where
	Self: 'a + Sized + Titled + NeatPrintable
{
	fn quick_find(data: impl Iterator<Item = &'a Self>) -> Option<&'a Self>;
}

impl<'a> Searchable<'a> for Purchase {
	fn quick_find(data: impl Iterator<Item = &'a Self>) -> Option<&'a Self> {
		let title = try_prompt_question(format!(
			"What is the title of the {}?",
			Self::type_name_pretty().to_lowercase()
		))?
		.to_lowercase();
		let title = title.trim();
		let mut found_matches: Vec<&Self> = data
			.filter(|purchase| purchase.title.to_lowercase().contains(title))
			.collect();
		match found_matches.len() {
			0 => None,
			1 => Some(found_matches.remove(0)),
			_ => {
				println!(
					"Multiple {} contained this in their title, looking for exact match...",
					Self::type_name_pretty().to_lowercase()
				);
				let exact_matches: Vec<&Purchase> = found_matches
					.clone()
					.into_iter()
					.filter(|purchase| purchase.title.trim() == title)
					.collect();
				match exact_matches.len() {
					1 => {
						println!("Found exactly one match with the exact same title, using that.");
						Some(exact_matches.first().unwrap())
					},
					len => {
						if len == 0 {
							println!("No matches found with exact title.");
						} else {
							println!("Many matches found with exact title.");
							found_matches = exact_matches;
						}
						println!("We must narrow our search using identifiers.");
						println!(
							"Identifiers present for {}s with this name:",
							Self::type_name_pretty().to_lowercase()
						);

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
							println!("Choose an identifier to narrow the search by.");
							'tag_specify_loop: loop {
								let replied_tag: Identifier = get_reply().into();
								let tag_recognized = found_matches
									.iter()
									.any(|found_match| found_match.has_identifier(&replied_tag));
								if !tag_recognized {
									println!(
										"Identifier was not found in the collection of matches."
									);
									println!("Try again.");
									continue 'tag_specify_loop;
								}
								found_matches
									.retain(|found_match| found_match.has_identifier(&replied_tag));
								break 'tag_specify_loop;
							}
							if found_matches.is_empty() {
								break 'tag_narrow_loop None;
							}
							if found_matches.len() == 1 {
								break 'tag_narrow_loop Some(found_matches.remove(0));
							}
							println!(
								"Multiple {}s share provided identifer(s) in the dataset.",
								Self::type_name_pretty().to_lowercase()
							);
							println!(
								"Identifiers present for {}s with specified name and identifiers:",
								Self::type_name_pretty().to_lowercase()
							);
						}
					}
				}
			}
		}
	}
}
impl<'a> Searchable<'a> for Rule {
	fn quick_find(data: impl Iterator<Item = &'a Self>) -> Option<&'a Self> {
		let title = prompt_question("What is the title of the rule?");
		// first try title direct match
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
}
