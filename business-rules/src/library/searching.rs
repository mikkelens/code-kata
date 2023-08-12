use std::{
	collections::{btree_set::Iter, BTreeSet},
	fmt::{Debug, Display}
};

use super::{
	io::try_prompt_question,
	printing::NeatPrintable,
	types::{Named, Purchase, Rule}
};
use crate::library::{io::get_reply, types::Identifier};

enum SearchMatches<T> {
	MultipleExact(Vec<T>),
	SingleExact(T),
	None,
	SinglePartial(T),
	MultiplePartial(Vec<T>)
}
impl<T: Debug + Display> SearchMatches<T> {
	pub fn print_evaluation(&self) {
		match self {
			SearchMatches::MultipleExact(_) => println!("Multiple exact matches found."),
			SearchMatches::SingleExact(_) => println!("Single exact match found."),
			SearchMatches::None => println!("No matches found."),
			SearchMatches::SinglePartial(_) => println!("Single partial match found."),
			SearchMatches::MultiplePartial(_) => println!("Multiple partial matches found.")
		};
		match self {
			SearchMatches::MultipleExact(values) | SearchMatches::MultiplePartial(values) => {
				println!("[{:?}]", values);
			},
			SearchMatches::SingleExact(value) | SearchMatches::SinglePartial(value) => {
				println!("{}", value);
			},
			SearchMatches::None => {}
		};
	}
}

pub trait Search<'a, T, U> {
	fn search_filter(self, value: U) -> SearchMatches<&'a T>;
}
impl<'a, T: 'a + Named, I: Iterator<Item = &'a T>> Search<'a, T, String> for I {
	fn search_filter(self, value: String) -> SearchMatches<&'a T> {
		let name = value;
		let mut exact_matches: Vec<&T> = self.filter(|item| item.name() == name).collect();
		match exact_matches.len() {
			0 => {
				let mut partial_matches: Vec<&T> = exact_matches
					.into_iter()
					.filter(|item| item.name().contains(&name))
					.collect();
				match partial_matches.len() {
					0 => SearchMatches::None,
					1 => SearchMatches::SinglePartial(partial_matches.remove(0)),
					_ => SearchMatches::MultiplePartial(partial_matches)
				}
			},
			1 => SearchMatches::SingleExact(exact_matches.remove(0)),
			_ => SearchMatches::MultipleExact(exact_matches)
		}
	}
}

pub trait Searchable<'a>
where
	Self: 'a + Sized + Named + NeatPrintable + Debug + Display
{
	fn try_find_single(data: impl Iterator<Item = &'a Self>) -> Option<&'a Self> {
		let title = try_prompt_question(format!(
			"What is the title of the {}?",
			Self::type_name_pretty().to_lowercase()
		))?
		.to_lowercase();
		let title_search = data.search_filter(title);
		title_search.print_evaluation();
		match title_search {
			SearchMatches::MultipleExact(multiple) | SearchMatches::MultiplePartial(multiple) => {
				Self::find_specific(multiple, &title)
			},
			SearchMatches::SingleExact(single) | SearchMatches::SinglePartial(single) => {
				Some(single)
			},
			SearchMatches::None => None
		}
	}
	fn find_specific(data: Vec<&Self>, searched_title: &str) -> Option<&'a Self>; // more specific than title
}

impl<'a> Searchable<'a> for Purchase {
	fn find_specific(data: Vec<&Self>, title: &str) -> Option<&'a Self> {
		let mut identifiers = data
			.iter()
			.flat_map(|item| &item.identifiers.0)
			.collect::<BTreeSet<&Identifier>>(); // unique set of all identifiers
		println!(
			"All identifiers for the search '{}': [{:?}]",
			title,
			identifiers.iter().collect::<Vec<_>>()
		);
		println!(
			"Provide one or more (with ';' as seperator) identifiers to narrow your search. \
			 (Unknown identifiers will be ignored)"
		);
		let identifier_strs = get_reply();
		for identifier_str in identifier_strs.split(';').map(str::trim) {
			let iter = identifiers.iter();
			iter.search_filter(identifier_str.to_string());
		}
		todo!()

		// 	let mut found_matches: Vec<&Self> = data.collect();
		// 	match found_matches.len() {
		// 		0 => None,
		// 		1 => Some(found_matches.remove(0)),
		// 		_ => {
		// 			print!(
		// 				"Multiple {} contained this in their title... ",
		// 				Self::type_name_pretty().to_lowercase()
		// 			);
		// 			let exact_matches: Vec<&Purchase> = found_matches
		// 				.clone()
		// 				.into_iter()
		// 				.filter(|purchase| purchase.title.trim() == title)
		// 				.collect();
		// 			match exact_matches.len() {
		// 				1 => {
		// 					println!("Found exactly one match with the exact same title, using
		// that."); 					Some(exact_matches.first().unwrap())
		// 				},
		// 				len => {
		// 					if len == 0 {
		// 					} else {
		// 						println!("Many matches found with exact title.");
		// 						found_matches = exact_matches;
		// 					}
		// 					println!("We must narrow our search using identifiers.");
		// 					println!(
		// 						"Identifiers present for {} in search:",
		// 						Self::type_name_pretty().to_lowercase()
		// 					);
		// 					'tag_narrow_loop: loop {
		// 						let unique_tags: BTreeSet<&Identifier> = found_matches
		// 							.iter()
		// 							.flat_map(|purchase| purchase.get_all_ídentifiers()) // iter of vecs
		// to vec 							.collect(); // set -> vec, means unique
		// 						println!(
		// 							"[{}] (unordered)",
		// 							unique_tags
		// 								.into_iter()
		// 								.map(|i| i.0.as_ref())
		// 								.collect::<Vec<_>>()
		// 								.join(", ")
		// 						);
		// 						println!("Choose an identifier to narrow the search by.");
		// 						'tag_specify_loop: loop {
		// 							let replied_tag: Identifier = get_reply().into();
		// 							let tag_recognized = found_matches
		// 								.iter()
		// 								.any(|found_match| found_match.has_identifier(&replied_tag));
		// 							if !tag_recognized {
		// 								println!(
		// 									"Identifier was not found in the collection of matches."
		// 								);

		// 								continue 'tag_specify_loop;
		// 							}
		// 							found_matches
		// 								.retain(|found_match| found_match.has_identifier(&replied_tag));
		// 							break 'tag_specify_loop;
		// 						}
		// 						if found_matches.is_empty() {
		// 							break 'tag_narrow_loop None;
		// 						}
		// 						if found_matches.len() == 1 {
		// 							break 'tag_narrow_loop Some(found_matches.remove(0));
		// 						}
		// 						println!(
		// 							"Multiple {}s share provided identifer(s) in the dataset.",
		// 							Self::type_name_pretty().to_lowercase()
		// 						);
		// 						println!(
		// 							"Identifiers present for {}s with specified name and identifiers:",
		// 							Self::type_name_pretty().to_lowercase()
		// 						);
		// 					}
		// 				}
		// 			}
		// 		}
		// 	}
	}
}
impl<'a> Searchable<'a> for Rule {
	fn find_specific(data: Vec<&Self>, _title: &str) -> Option<&'a Self> {
		todo!()
		// let mut found_matches: Vec<&Rule> = data.collect();
		// if found_matches.is_empty() {
		// 	None
		// } else if found_matches.len() == 1 {
		// 	Some(found_matches.remove(0))
		// } else {
		// 	println!("Rules with the same name were found in the dataset.");
		// 	for (index, rule) in found_matches.iter().enumerate() {
		// 		println!("Rule {} process_action: {}", index, rule.process_action);
		// 	}
		// 	println!("Choose a rule using its index (number to the left).");
		// 	let selected_match = 'index_request_loop: loop {
		// 		let reply = get_reply();
		// 		let Ok(unsigned_int_reply) = reply.trim().parse::<usize>() else {
		// 			println!("Reply was not an unsigned integer.");
		// 			println!("Try again.");
		// 			continue 'index_request_loop;
		// 		};
		// 		if unsigned_int_reply >= found_matches.len() {
		// 			println!("Reply index was not inside the range of the found
		// matches."); 			println!("Try again.");
		// 			continue 'index_request_loop;
		// 		}
		// 		break 'index_request_loop found_matches.remove(unsigned_int_reply);
		// 	};
		// 	Some(selected_match)
		// }
	}
}
