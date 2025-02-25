use std::collections::BTreeMap;

const MAX_ORDERS_PER_LEVEL: usize = 32;

#[derive(Clone, Copy, Debug, Default)]
struct Order {
    size: u64,
}

#[derive(Debug)]
struct PriceLevelOrders {
    orders: [Option<Order>; MAX_ORDERS_PER_LEVEL],
    count: usize,
}

impl PriceLevelOrders {
    fn new() -> Self {
        Self {
            orders: [None; MAX_ORDERS_PER_LEVEL],
            count: 0,
        }
    }

    fn insert(&mut self, order: Order) -> bool {
        if self.count >= MAX_ORDERS_PER_LEVEL {
            return false;
        }
        let mut idx = 0;
        while idx < self.count {
            if let Some(existing) = self.orders[idx] {
                if order.size > existing.size {
                    break;
                }
            }
            idx += 1;
        }
        for j in (idx..self.count).rev() {
            self.orders[j + 1] = self.orders[j];
        }
        self.orders[idx] = Some(order);
        self.count += 1;
        true
    }

    fn remove(&mut self, size: u64) -> Option<Order> {
        for i in 0..self.count {
            if let Some(order) = self.orders[i] {
                if order.size == size {
                    let removed = order;
                    // Shift orders left to fill the gap.
                    for j in i..self.count - 1 {
                        self.orders[j] = self.orders[j + 1];
                    }
                    self.orders[self.count - 1] = None;
                    self.count -= 1;
                    return Some(removed);
                }
            }
        }
        None
    }

    fn best_order(&self) -> Option<Order> {
        self.orders.get(0).and_then(|&o| o)
    }
}

pub struct OrderBook {
    bids: BTreeMap<u64, PriceLevelOrders>,
    asks: BTreeMap<u64, PriceLevelOrders>,
}

impl OrderBook {
    fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    fn add_order(&mut self, price: u64, size: u64, is_bid: bool) -> bool {
        let book = if is_bid {
            &mut self.bids
        } else {
            &mut self.asks
        };
        let level = book.entry(price).or_insert_with(PriceLevelOrders::new);
        let order = Order { size };
        level.insert(order)
    }

    fn remove_order(&mut self, price: u64, size: u64, is_bid: bool) -> Option<Order> {
        let book = if is_bid {
            &mut self.bids
        } else {
            &mut self.asks
        };
        if let Some(level) = book.get_mut(&price) {
            let order = level.remove(size);
            if level.count == 0 {
                book.remove(&price);
            }
            order
        } else {
            None
        }
    }

    fn best_bid(&self) -> Option<(u64, Order)> {
        self.bids
            .iter()
            .next_back()
            .and_then(|(&price, level)| level.best_order().map(|order| (price, order)))
    }

    fn best_ask(&self) -> Option<(u64, Order)> {
        self.asks
            .iter()
            .next()
            .and_then(|(&price, level)| level.best_order().map(|order| (price, order)))
    }
}

#[cfg(test)]
mod tests {
    use crate::treebook::OrderBook;

    #[test]
    fn test0() {
        let mut order_book = OrderBook::new();

        order_book.add_order(100, 10, true);
        order_book.add_order(100, 15, true);
        order_book.add_order(100, 5, true);

        order_book.add_order(101, 8, false);
        order_book.add_order(101, 12, false);

        println!("Best Bid: {:?}", order_book.best_bid());
        println!("Best Ask: {:?}", order_book.best_ask());
    }
}
