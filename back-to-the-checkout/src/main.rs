#![allow(unused)]

use std::{
	cmp::Ordering,
	collections::{hash_map, HashMap},
	ops::Deref,
	sync::Arc
};

use hash_map::Entry;

fn main() {
	let bean_name: ItemName = Arc::from("Can of Beans");
	let banana_name: ItemName = Arc::from("Banana");
	let tomato_name: ItemName = Arc::from("Tomato");
	let milk_name: ItemName = Arc::from("Milk");
	let mut store = Store::from_iter([
		(bean_name.as_ref(), 10),
		(banana_name.as_ref(), 12),
		(tomato_name.as_ref(), 9),
		(milk_name.as_ref(), 6)
	]);

	store
		.scan(&bean_name)
		.expect("there is at least 1 bean can at this point");
	store
		.scan(&bean_name)
		.expect("there is at least 1 bean can at this point");
	store
		.scan(&bean_name)
		.expect("there is at least 1 bean can at this point");
	store
		.scan(&banana_name)
		.expect("there is at least 1 bean can at this point");
	store
		.scan(&banana_name)
		.expect("there is at least 1 bean can at this point");
	store
		.scan(&bean_name)
		.expect("there is at least 1 bean can at this point");
	store
		.scan_multiple(&milk_name, 5)
		.expect("should be able to do this many");
	store
		.unscan_multiple(&milk_name, 1)
		.expect("there is one to unscan");
	store
		.scan_multiple(&milk_name, 2)
		.expect("there is space again");
	store
		.unscan_multiple(&bean_name, 4)
		.expect("removed all at once");
	store.unscan(&bean_name).expect_err("cannot remove another");
	store.scan_multiple(&bean_name, 10).expect("took them all");
	store
		.scan_multiple(&bean_name, 320)
		.expect("can take an unlimited amount");
	let price = store.get_checkout_price();
	let checkout_items = store.complete_checkout();
	println!("Items checked out: {:?}", checkout_items);
	println!("Store after checked out: {:?}", store);
}

type ItemName = Arc<str>;

type UnsignedAmount = usize;
type SignedAmount = isize;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct UnsignedMoneyValue(UnsignedAmount);
impl From<usize> for UnsignedMoneyValue {
	fn from(value: usize) -> Self {
		UnsignedMoneyValue(value)
	}
}

#[derive(Debug, Clone, PartialEq)]
struct Deal {
	item_amount:      UnsignedAmount,
	price_for_amount: UnsignedMoneyValue
}

#[derive(Debug, Clone, PartialEq)]
struct ItemPrice {
	unit:    UnsignedMoneyValue,
	special: Option<Deal> // optimal deals should always be cheaper than unit price!
}

#[derive(Debug, Clone, PartialEq)]
enum StockType {
	Unlimited,
	Limited(UnsignedAmount)
}
impl PartialOrd for StockType {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(if self == other {
			Ordering::Equal
		} else {
			match self {
				StockType::Unlimited => Ordering::Greater,
				StockType::Limited(self_amount) => match other {
					StockType::Unlimited => Ordering::Less,
					StockType::Limited(other_amount) => self_amount.partial_cmp(other_amount)?
				}
			}
		})
	}
}

#[derive(Debug, Clone, PartialEq)]
struct ItemData {
	stock: StockType,
	price: ItemPrice
}
impl ItemData {
	fn basic(cost: impl Into<UnsignedMoneyValue>) -> Self {
		ItemData {
			stock: StockType::Unlimited,
			price: ItemPrice {
				unit:    cost.into(),
				special: None
			}
		}
	}
}

#[derive(Debug, PartialEq)]
enum CheckoutModifyError {
	MissingAmount(UnsignedAmount),
	MissingItem(ItemName)
}

#[derive(Debug, Default, PartialEq)]
struct Checkout {
	items: HashMap<ItemName, UnsignedAmount>
}

#[derive(Debug, Default, PartialEq)]
struct Store {
	stock_keeping_units: HashMap<ItemName, ItemData>, // name -> stock (can be infinite) & price
	active_checkout:     Checkout                     // mutable data
}

#[derive(Debug)]
enum StoreQueryError {
	MissingItem(ItemName)
}

#[allow(unused)]
impl Store {
	fn get_checkout_price(&self) -> Result<UnsignedMoneyValue, StoreQueryError> {
		Ok(UnsignedMoneyValue(
			self.active_checkout
				.items
				.iter()
				.map(|(item, amount)| {
					(
						self.stock_keeping_units
							.get(item.clone().as_ref())
							.ok_or(StoreQueryError::MissingItem(item.clone())),
						amount
					)
				})
				.collect::<Result<_, StoreQueryError>>()?
				.into_iter()
				.map(|(price, amount)| amount * price)
				.sum()
		))
	}

	fn complete_checkout(&mut self) -> Checkout {
		todo!()
	}

	fn add_item(&mut self, item: ItemName, data: ItemData) -> Option<ItemName> {
		if let Entry::Vacant(e) = self.stock_keeping_units.entry(item.clone()) {
			e.insert(data);
			None
		} else {
			Some(item)
		}
	}

	fn remove_item(&mut self, item: &ItemName) -> Option<(ItemName, ItemData)> {
		if let Entry::Occupied(e) = self.stock_keeping_units.entry(item.clone()) {
			Some(e.remove_entry())
		} else {
			None
		}
	}

	fn scan(&mut self, item: &ItemName) -> Result<(), CheckoutModifyError> {
		self.scan_multiple(item, 1)
	}

	fn scan_multiple(
		&mut self,
		item: &ItemName,
		amount: UnsignedAmount
	) -> Result<(), CheckoutModifyError> {
		let checkout_amount = self.active_checkout.items.entry(item.clone()).or_insert(0);
		if let StockType::Limited(stock_amount) = self
			.stock_keeping_units
			.get_mut(item)
			.ok_or(CheckoutModifyError::MissingItem(item.clone()))?
			.stock
		{
			debug_assert!(stock_amount >= *checkout_amount); // should never have more items than expected
			let combined_amount = *checkout_amount + amount;
			if stock_amount.cmp(&combined_amount) == Ordering::Less {
				return Err(CheckoutModifyError::MissingAmount(
					combined_amount - stock_amount
				));
			}
		}

		*checkout_amount += amount;
		Ok(())
	}

	fn unscan(&mut self, item: &ItemName) -> Result<(), CheckoutModifyError> {
		self.unscan_multiple(item, 1)
	}

	fn unscan_multiple(
		&mut self,
		item: &ItemName,
		amount: UnsignedAmount
	) -> Result<(), CheckoutModifyError> {
		let checkout_amount = *self
			.active_checkout
			.items
			.get(item)
			.ok_or(CheckoutModifyError::MissingItem(item.clone()))?;
		debug_assert!(checkout_amount > 0); // if not missing, should be at least 1
		match (checkout_amount).cmp(&amount) {
			Ordering::Less => {
				// not enough to remove all items
				return Err(CheckoutModifyError::MissingAmount(amount - checkout_amount));
			},
			Ordering::Equal => {
				// all elements removed exactly
				let _ = self.active_checkout.items.remove(item);
			},
			Ordering::Greater => *self.active_checkout.items.get_mut(item).unwrap() -= amount
		}
		Ok(())
	}

	fn unscan_all(&mut self, item: &ItemName) -> Result<(UnsignedAmount), CheckoutModifyError> {
		self.active_checkout
			.items
			.remove(item)
			.ok_or(CheckoutModifyError::MissingItem(item.clone()))
	}
}
impl<T: AsRef<str>, U: Clone + Into<UnsignedMoneyValue>> FromIterator<(T, U)> for Store {
	fn from_iter<I: IntoIterator<Item = (T, U)>>(iter: I) -> Self {
		Store {
			stock_keeping_units: HashMap::from_iter(
				iter.into_iter()
					.map(|(name, cost)| (name.as_ref().into(), ItemData::basic(cost.into())))
			),
			active_checkout:     Checkout {
				items: Default::default()
			}
		}
	}
}