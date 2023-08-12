use std::{any::type_name, collections::BTreeSet, fmt::Display};

use super::{user_creation::UserSelected, ApplicationData, PathFindable};
use crate::library::{
	io::Saved,
	searching::Searchable,
	types::{Order, Purchase, Rule}
};

pub trait NeatPrintable {
	fn print(&self);
	fn type_name_pretty() -> &'static str {
		let raw_name = type_name::<Self>();
		if let Some(simplified_name) = raw_name.split("::").last() {
			simplified_name
		} else {
			raw_name
		}
	}
}
impl NeatPrintable for Purchase {
	fn print(&self) {
		println!("Purchase:\n{}", self);
	}
}
impl NeatPrintable for Rule {
	fn print(&self) {
		println!("Rule:\n{}", self);
	}
}

impl NeatPrintable for Order {
	fn print(&self) {
		if self.purchases.0.is_empty() {
			println!("No purchases in order to print.");
		} else {
			for (index, (purchase, amount)) in self.purchases.0.iter().enumerate() {
				println!("Purchase no. {} (x{}):\n{}", index + 1, amount, purchase);
				println!();
			}
		}
	}
}
pub(crate) fn print_purchase_data_order(data: &ApplicationData) {
	Order::prompt_data_selection(data).print();
}
impl<T: Display> NeatPrintable for BTreeSet<T> {
	fn print(&self) {
		for (index, item) in self.iter().enumerate() {
			println!("{} no. {}:\n{}", Self::type_name_pretty(), index + 1, item);
			println!();
		}
	}
}

pub(crate) fn print_processing_individual(data: &ApplicationData) {
	let Ok(rules) = Rule::load_from_disk_retrying(Rule::get_path(data)) else {
		return;
	};
	if rules.is_empty() {
		println!("There are currently no rules to trigger any processes.");
	} else {
		let Ok(purchases) = Purchase::load_from_disk_retrying(Purchase::get_path(data)) else {
			return;
		};
		let possible_purchase = Purchase::try_find_single(purchases.iter());
		println!();
		if let Some(purchase) = possible_purchase {
			let processing_steps = purchase.get_processing_steps(&rules);
			if processing_steps.is_empty() {
				println!("This purchase does trigger any processing rules.");
			} else {
				println!(
					"The processing steps for this purchase are the following:\n - {}",
					processing_steps
						.iter()
						.map(AsRef::as_ref)
						.collect::<Vec<_>>()
						.join("\n - ")
				);
			}
		} else {
			println!("No item with the provided specifications could be found.");
		}
	}
}
pub(crate) fn print_processing_order(data: &ApplicationData) {
	let order = Order::prompt_data_selection(data);
	println!(); // post-user-entry spacing
	if order.purchases.0.is_empty() {
		println!("No purchases in order to print.");
	} else {
		let Ok(rules) = Rule::load_from_disk_retrying(Rule::get_path(data)) else {
			return;
		};

		if rules.is_empty() {
			println!("There are currently no rules to trigger any processes.");
		} else {
			let rules = &rules;
			for (index, (purchase, amount)) in order.purchases.0.iter().enumerate() {
				println!("Purchase no. {} (x{}):\n{}", index + 1, amount, purchase);
				print_processing_steps(purchase, rules);
				println!(); // extra spacing between each
			}
		}
	}
}

pub(crate) fn print_processing_all(data: &ApplicationData) {
	let Ok(all_purchases) = Purchase::load_from_disk_retrying(Purchase::get_path(data)) else {
		return;
	};
	let Ok(rules) = Rule::load_from_disk_retrying(Rule::get_path(data)) else {
		return;
	};
	if rules.is_empty() {
		println!("There are currently no rules to trigger any processes.");
	} else {
		let rules = &rules;
		for (index, purchase) in all_purchases.iter().enumerate() {
			println!("Purchase no. {}:\n{}", index + 1, purchase);
			print_processing_steps(purchase, rules);
			println!();
		}
	}
}

pub(crate) fn print_processing_steps(purchase: &Purchase, rules: &BTreeSet<Rule>) {
	let processing_steps = purchase.get_processing_steps(rules);
	if processing_steps.is_empty() {
		println!("This purchase does trigger any processing rules.");
	} else {
		println!(
			"The processing steps for this purchase are the following:\n - {}",
			processing_steps
				.iter()
				.map(AsRef::as_ref)
				.collect::<Vec<_>>()
				.join("\n - ")
		);
	}
}
