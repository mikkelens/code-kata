use std::{
	collections::BTreeSet,
	fmt::{Debug, Display}
};

use super::{
	io::try_prompt_question,
	printing::NeatPrintable,
	types::{Named, Purchase, Rule}
};
use crate::library::{io::get_reply, types::Identifier};

enum SearchMatches<T, I> {
	MultipleExact(I),
	SingleExact(T),
	None,
	SinglePartial(T),
	MultiplePartial(I)
}
impl<T, I> SearchMatches<T, I>
where
	T: Display,
	I: Debug
{
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

trait NameSearchable<'a, T>
where
	T: 'a
{
	fn filter_by_name<I>(value: &str, iter: I) -> SearchMatches<&'a T, Vec<&'a T>>
	where
		I: Iterator<Item = &'a T>;
}
impl<'a, T> NameSearchable<'a, T> for T
where
	T: 'a + Named
{
	fn filter_by_name<I>(name: &str, iter: I) -> SearchMatches<&'a T, Vec<&'a T>>
	where
		I: Iterator<Item = &'a T>
	{
		let data: Vec<&'a T> = iter.collect();
		let mut exact_matches: Vec<&'a T> = data
			.iter()
			.copied()
			.filter(|item| item.name() == name)
			.collect();
		match exact_matches.len() {
			0 => {
				drop(exact_matches);
				let name_lowercase = name.to_lowercase();
				let mut lower_matches: Vec<&'a T> = data
					.iter()
					.copied()
					.filter(|item| item.name().to_lowercase() == name_lowercase)
					.collect();
				match lower_matches.len() {
					0 => {
						drop(lower_matches);
						let mut partial_lower_matches: Vec<&'a T> = data
							.into_iter()
							.filter(|item| item.name().to_lowercase().contains(&name_lowercase))
							.collect();
						match partial_lower_matches.len() {
							0 => SearchMatches::None,
							1 => SearchMatches::SinglePartial(partial_lower_matches.remove(0)),
							_ => SearchMatches::MultiplePartial(partial_lower_matches)
						}
					},
					1 => SearchMatches::SingleExact(lower_matches.remove(0)),
					_ => SearchMatches::MultipleExact(lower_matches)
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
	fn try_find_single<I>(data: I) -> Option<&'a Self>
	where
		I: Iterator<Item = &'a Self>
	{
		let title = try_prompt_question(format!(
			"What is the title of the {}?",
			Self::type_name_pretty().to_lowercase()
		))?;
		let title_search = Self::filter_by_name(title.as_str(), data);
		title_search.print_evaluation();
		match title_search {
			SearchMatches::MultipleExact(multiple) | SearchMatches::MultiplePartial(multiple) => {
				Self::find_specific(multiple, title.as_str())
			},
			SearchMatches::SingleExact(single) | SearchMatches::SinglePartial(single) => {
				Some(single)
			},
			SearchMatches::None => None
		}
	}
	fn find_specific(data: Vec<&'a Self>, searched_title: &str) -> Option<&'a Self>; // more specific than title
}

impl<'a> Searchable<'a> for Purchase {
	fn find_specific(mut data: Vec<&'a Self>, searched_title: &str) -> Option<&'a Self> {
		let identifiers = data
			.iter()
			.flat_map(|item| &item.identifiers.0)
			.collect::<BTreeSet<&Identifier>>(); // unique set of all identifiers
		println!(
			"All identifiers for the search '{}': [{:?}]",
			searched_title,
			identifiers.iter().collect::<Vec<_>>()
		);

		'identifier_narrow: loop {
			let identifier_to_narrow_with = 'identifier_filter_select: loop {
				println!("Provide one or more (';' separated) identifiers to narrow your search.");
				let answer = get_reply();
				let identifier_search =
					Identifier::filter_by_name(&answer, identifiers.clone().into_iter());
				break 'identifier_filter_select match identifier_search {
					SearchMatches::MultipleExact(_multiple) => {
						unreachable!("It is impossible for two identifiers to have the same name.");
					},
					SearchMatches::SingleExact(single) | SearchMatches::SinglePartial(single) => {
						println!("Found matching identifier '{}'.", single);
						single
					},
					SearchMatches::None => {
						println!("Identifier '{}' was not found, try again.", answer);
						continue 'identifier_filter_select;
					},
					SearchMatches::MultiplePartial(multiple) => {
						println!(
							"Multiple identifiers found from partial '{}': {:?}, try again.",
							answer, multiple
						);
						continue 'identifier_filter_select;
					}
				};
			};
			data.retain(|purchase| purchase.has_identifier(identifier_to_narrow_with));
			match data.len() {
				0 => unreachable!(
					"Impossible to have no data remaining since only one is removed at a time."
				),
				1 => break 'identifier_narrow Some(data.remove(0)),
				_ => continue 'identifier_narrow
			}
		}
	}
}
impl<'a> Searchable<'a> for Rule {
	fn find_specific(mut found_matches: Vec<&'a Self>, _searched_title: &str) -> Option<&'a Self> {
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
