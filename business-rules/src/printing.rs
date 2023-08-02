use std::collections::BTreeSet;

use crate::{
	data::{
		io::{load_purchases, load_rules},
		types::{Order, Purchase, Rule, UserCreated}
	},
	quick_find_purchase, quick_find_rule
};

pub fn print_purchase_data_individual() {
	let purchases = load_purchases();
	let possible_purchase = quick_find_purchase(purchases.iter());
	println!();
	if let Some(purchase) = possible_purchase {
		println!("Purchase:\n{}", purchase);
	} else {
		println!("No purchase with the provided specifications could be found.");
	}
}
pub fn print_rule_data_individual() {
	let rules = load_rules();
	let possible_rule = quick_find_rule(rules.iter());
	println!();
	if let Some(rule) = possible_rule {
		println!("Purchase:\n{}", rule);
	} else {
		println!("No rule with the provided specifications could be found.");
	}
}
pub fn print_purchase_data_order() {
	let order = Order::prompt_creation();
	println!();
	if order.purchases.0.is_empty() {
		println!("No purchases in order to print.");
	} else {
		for (index, (purchase, amount)) in order.purchases.0.iter().enumerate() {
			println!("Purchase no. {} (x{}):\n{}", index + 1, amount, purchase);
			println!();
		}
	}
}
pub fn print_purchase_data_all() {
	let all_purchases = load_purchases();
	for (index, purchase) in all_purchases.iter().enumerate() {
		println!("Purchase no. {}:\n{}", index + 1, purchase);
		println!();
	}
}
pub fn print_rule_data_all() {
	let all_rules = load_rules();
	for (index, rule) in all_rules.iter().enumerate() {
		println!("Rule no. {}:\n{}", index + 1, rule);
		println!();
	}
}

pub fn print_processing_individual() {
	let rules = load_rules();
	if rules.is_empty() {
		println!("There are currently no rules to trigger any processes.");
	} else {
		let purchases = load_purchases();
		let possible_purchase = quick_find_purchase(purchases.iter());
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
pub fn print_processing_order() {
	let order = Order::prompt_creation();
	println!(); // post-user-entry spacing
	if order.purchases.0.is_empty() {
		println!("No purchases in order to print.");
	} else {
		let rules = load_rules();
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

pub fn print_processing_all() {
	let all_purchases = load_purchases();
	let rules = load_rules();
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

pub fn print_processing_steps(purchase: &Purchase, rules: &BTreeSet<Rule>) {
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
