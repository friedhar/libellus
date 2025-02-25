const MAX_ORDERS: usize = 1024;
#[derive(Copy, Clone, PartialEq)]
struct Order {
    px: u64,
    sz: u32,
    ts: u64,
    id: u32,
}

struct OrderBook {
    bids: [Option<Order>; MAX_ORDERS],
    asks: [Option<Order>; MAX_ORDERS],
    bid_count: usize,
    ask_count: usize,
    next_id: u32,
}

impl OrderBook {
    #[inline]
    fn new() -> Self {
        OrderBook {
            bids: [None; MAX_ORDERS],
            asks: [None; MAX_ORDERS],
            bid_count: 0,
            ask_count: 0,
            next_id: 0,
        }
    }

    #[inline]
    fn place(&mut self, px: u64, sz: u32, is_bid: bool, ts: u64) -> Option<u32> {
        if (is_bid && self.bid_count >= MAX_ORDERS) || (!is_bid && self.ask_count >= MAX_ORDERS) {
            return None;
        }

        let order = Order {
            px,
            sz,
            ts,
            id: self.next_id,
        };
        self.next_id = self.next_id.wrapping_add(1);

        let (orders, count) = if is_bid {
            (&mut self.bids, &mut self.bid_count)
        } else {
            (&mut self.asks, &mut self.ask_count)
        };

        let mut i = 0;
        while i < *count {
            let Some(existing) = orders[i] else { break };
            if Self::compare_orders(&order, &existing, is_bid) {
                break;
            }
            i += 1;
        }

        if i < *count {
            orders.copy_within(i..*count, i + 1);
        }

        orders[i] = Some(order);
        *count += 1;
        Some(order.id)
    }

    #[inline]
    fn compare_orders(new: &Order, existing: &Order, is_bid: bool) -> bool {
        if new.px != existing.px {
            if is_bid {
                new.px > existing.px
            } else {
                new.px < existing.px
            }
        } else if new.sz != existing.sz {
            new.sz > existing.sz
        } else {
            new.ts < existing.ts
        }
    }

    #[inline]
    fn cancel(&mut self, id: u32, is_bid: bool) -> bool {
        let (orders, count) = if is_bid {
            (&mut self.bids, &mut self.bid_count)
        } else {
            (&mut self.asks, &mut self.ask_count)
        };

        for i in 0..*count {
            if let Some(order) = orders[i] {
                if order.id == id {
                    if i < *count - 1 {
                        orders.copy_within(i + 1..*count, i);
                    }
                    orders[*count - 1] = None;
                    *count -= 1;
                    return true;
                }
            }
        }
        false
    }

    #[inline]
    fn best_bid(&self) -> Option<Order> {
        if self.bid_count > 0 {
            self.bids[0].clone()
        } else {
            None
        }
    }

    #[inline]
    fn best_ask(&self) -> Option<Order> {
        if self.ask_count > 0 {
            self.asks[0].clone()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_book() {
        let book = OrderBook::new();
        assert!(book.best_bid().is_none());
        assert!(book.best_ask().is_none());
        assert_eq!(book.bid_count, 0);
        assert_eq!(book.ask_count, 0);
    }
}
