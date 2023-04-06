#[repr(transparent)]
#[derive(Debug)]
struct DivisibleMoney(f32);

#[derive(Debug)]
struct PriceWithSale {
    single_cost: DivisibleMoney,
    multiple_cost: DivisibleMoney,
    multiple_amount: usize,
}
impl PriceWithSale {
    fn total_cost_with_sale(&self, amount: &usize) -> DivisibleMoney {
        if amount < &self.multiple_amount {
            return DivisibleMoney(self.single_cost.0 * *amount as f32);
        }
        let sale_multiples: usize = amount / self.multiple_amount;
        let left_over_amount = amount % self.multiple_amount;

        let sale_price_total = self.multiple_cost.0 * sale_multiples as f32;
        let left_over_total = self.single_cost.0 * left_over_amount as f32;

        DivisibleMoney(sale_price_total + left_over_total)
    }
}

#[derive(Debug)]
struct PriceWithBonus {
    single_cost: DivisibleMoney,
    amount_for_bonus: usize,
    bonus_count: usize,
}
impl PriceWithBonus {
    fn total_cost_bonuses_removed(&self, amount: &usize) -> DivisibleMoney {
        if amount < &self.amount_for_bonus {
            return DivisibleMoney(self.single_cost.0 * *amount as f32);
        }
        let bonus_bunch_size = self.amount_for_bonus + self.bonus_count;
        let bonus_bunches = amount / bonus_bunch_size;
        let left_over_amount = amount % bonus_bunch_size;

        let bunch_paid_count = bonus_bunches * self.amount_for_bonus;
        let total_paid_count = bunch_paid_count + left_over_amount;

        DivisibleMoney(total_paid_count as f32 * self.single_cost.0)
    }
}

#[derive(Debug)]
enum PricePerItem {
    Simple(DivisibleMoney),
    WithSale(PriceWithSale),
    WithBonus(PriceWithBonus),
}

#[derive(Debug)]
struct Item {
    #[allow(unused)]
    name: &'static str,
    price: PricePerItem,
}

#[derive(Debug)]
struct ItemCollection {
    item: Item,
    amount: usize,
}
impl ItemCollection {
    fn collection_total(&self) -> DivisibleMoney {
        match &self.item.price {
            PricePerItem::Simple(pis) => DivisibleMoney(pis.0 * self.amount as f32),
            PricePerItem::WithSale(piws) => piws.total_cost_with_sale(&self.amount),
            PricePerItem::WithBonus(piwb) => piwb.total_cost_bonuses_removed(&self.amount),
        }
    }
}

use test_stock::*;

fn main() {
    println!("Stock: {:?}", ENTIRE_STOCK);
    let stock_total: f32 = ENTIRE_STOCK.iter().map(|ic| ic.collection_total().0).sum();
    println!("Stock total: {}", stock_total);
}

mod test_stock {
    use super::*;

    // potato section
    pub(crate) const POTATO: Item = Item {
        name: "potato bag",
        price: PricePerItem::Simple(DivisibleMoney(2.0_f32)),
    };
    pub(crate) const POTATO_COLLECTION: ItemCollection = ItemCollection {
        item: POTATO,
        amount: 5,
    };

    // milk section
    pub(crate) const MILK: Item = Item {
        name: "milk carton",
        price: PricePerItem::WithSale(PriceWithSale {
            single_cost: DivisibleMoney(1.45_f32),
            multiple_cost: DivisibleMoney(2.15_f32),
            multiple_amount: 2,
        }),
    };
    pub(crate) const MILK_COLLECTION: ItemCollection = ItemCollection {
        item: MILK,
        amount: 5,
    };

    // nutella section
    pub(crate) const NUTELLA: Item = Item {
        name: "nutella jar",
        price: PricePerItem::WithBonus(PriceWithBonus {
            single_cost: DivisibleMoney(3.05_f32),
            amount_for_bonus: 2,
            bonus_count: 1,
        }),
    };
    pub(crate) const NUTELLA_COLLECTION: ItemCollection = ItemCollection {
        item: NUTELLA,
        amount: 4,
    };

    pub(crate) const ENTIRE_STOCK: &[ItemCollection] =
        &[POTATO_COLLECTION, MILK_COLLECTION, NUTELLA_COLLECTION];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_works() {
        let potato_collection_total = POTATO_COLLECTION.collection_total().0;
        assert_eq!(10_f32, potato_collection_total);
    }

    #[test]
    fn sale_works() {
        let milk_collection_total = MILK_COLLECTION.collection_total().0;
        assert_eq!(5.75_f32, milk_collection_total);
    }

    #[test]
    fn bonus_works() {
        let nutella_collection_total = NUTELLA_COLLECTION.collection_total().0;
        assert_eq!(9.15_f32, nutella_collection_total);
    }
}
