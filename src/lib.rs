use std::mem::MaybeUninit;

pub struct OrderBook<const N: usize> {
    bids: [MaybeUninit<Order>; N],
    asks: [MaybeUninit<Order>; N],
}

// u64 -- good for alignment, better than floats, also for arithmetic..
pub struct Order {
    px: u64,
    sz: u64,
}

impl Order {
    pub fn new(px: u64, sz: u64) -> Order {
        Order { px, sz }
    }
}

impl<const N: usize> OrderBook<N> {
    pub fn new() -> OrderBook<N> {
        OrderBook {
            bids: [const { MaybeUninit::uninit() }; N],
            asks: [const { MaybeUninit::uninit() }; N],
        }
    }

    pub fn place(&mut self, order: Order, bid: bool) {}
}
