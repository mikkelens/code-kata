use std::collections::BTreeSet;

use super::{
	io::prompt_question,
	types::{Purchase, Rule}
};
use crate::library::{io::get_reply, types::Identifier};

pub trait Searchable<'a>
where
	Self: Sized + 'a
{
	fn quick_find(data: impl Iterator<Item = &'a Self>) -> Option<&'a Self>;
}

impl<'a> Searchable<'a> for Purchase {
	fn quick_find(data: impl Iterator<Item = &'a Self>) -> Option<&'a Self> {
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
}
impl<'a> Searchable<'a> for Rule {
	fn quick_find(data: impl Iterator<Item = &'a Self>) -> Option<&'a Self> {
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
}
