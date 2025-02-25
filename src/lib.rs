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
            self.bids[0]
        } else {
            None
        }
    }

    #[inline]
    fn best_ask(&self) -> Option<Order> {
        if self.ask_count > 0 {
            self.asks[0]
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, hint::black_box, time::Instant};

    use super::*;

    #[test]
    fn test_bench0() {
        let k = 1e4 as usize;
        let n = 1e3 as usize;
        let mut pps_sum = 0.0;
        for _ in 0..k {
            let mut book = OrderBook::new();
            let t0 = Instant::now();
            for _ in 0..n {
                black_box(book.place(12, 1, true, 32489324));
            }
            let took_s = t0.elapsed().as_micros() as f64 / 1e6;
            let pps = n as f64 / took_s;
            pps_sum += pps;
        }
        println!(
            "iterations={k}, sample={n}, avg Place / S: {:.2}",
            pps_sum / k as f64
        );
    }

    #[test]
    fn test_empty_book() {
        let book = OrderBook::new();
        assert!(book.best_bid().is_none());
        assert!(book.best_ask().is_none());
        assert_eq!(book.bid_count, 0);
        assert_eq!(book.ask_count, 0);
    }

    #[test]
    fn test_place_and_best_px() {
        let mut book = OrderBook::new();

        let bid1 = book.place(10000, 50, true, 1).unwrap();
        let bid2 = book.place(9950, 100, true, 2).unwrap();
        let ask1 = book.place(10100, 75, false, 3).unwrap();
        let ask2 = book.place(10200, 25, false, 4).unwrap();

        let best_bid = book.best_bid().unwrap();
        assert_eq!(best_bid.px, 10000);
        assert_eq!(best_bid.sz, 50);
        assert_eq!(best_bid.id, bid1);

        let best_ask = book.best_ask().unwrap();
        assert_eq!(best_ask.px, 10100);
        assert_eq!(best_ask.sz, 75);
        assert_eq!(best_ask.id, ask1);
    }

    #[test]
    fn test_sorting_px_sz_ts() {
        let mut book = OrderBook::new();

        book.place(10000, 50, true, 1).unwrap();
        book.place(9950, 75, true, 1).unwrap();
        book.place(10000, 100, true, 1).unwrap();

        assert_eq!(book.bids[0].unwrap().px, 10000);
        assert_eq!(book.bids[0].unwrap().sz, 100);
        assert_eq!(book.bids[1].unwrap().px, 10000);
        assert_eq!(book.bids[1].unwrap().sz, 50);
        assert_eq!(book.bids[2].unwrap().px, 9950);

        let mut book = OrderBook::new();
        book.place(10000, 50, true, 3).unwrap();
        book.place(10000, 50, true, 1).unwrap();
        book.place(10000, 50, true, 2).unwrap();
        assert_eq!(book.bids[0].unwrap().ts, 1);
        assert_eq!(book.bids[1].unwrap().ts, 2);
        assert_eq!(book.bids[2].unwrap().ts, 3);
    }

    #[test]
    fn test_cancel() {
        let mut book = OrderBook::new();
        let bid1 = book.place(10000, 50, true, 1).unwrap();
        let bid2 = book.place(9950, 100, true, 2).unwrap();
        assert_eq!(book.bid_count, 2);
        assert!(book.cancel(bid1, true));
        assert_eq!(book.bid_count, 1);
        assert_eq!(book.best_bid().unwrap().px, 9950);
        assert!(!book.cancel(bid1, true));
        assert!(book.cancel(bid2, true));
        assert_eq!(book.bid_count, 0);
        assert!(book.best_bid().is_none());
    }

    #[test]
    fn test_capacity_limit() {
        let mut book = OrderBook::new();
        for i in 0..(MAX_ORDERS - 1) {
            assert!(book.place(10000 + i as u64, 50, true, 1).is_some());
        }
        assert!(book.place(20000, 50, true, 1).is_some());
        assert!(book.place(20001, 50, true, 1).is_none());
        assert!(book.place(30000, 50, false, 1).is_some());
    }

    #[test]
    fn test_id_uniqueness() {
        let mut book = OrderBook::new();
        let id1 = book.place(10000, 50, true, 1).unwrap();
        let id2 = book.place(9950, 100, true, 2).unwrap();
        let id3 = book.place(10100, 75, false, 3).unwrap();
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
    }
}
