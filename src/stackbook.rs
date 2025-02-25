use std::mem::MaybeUninit;

pub struct StackBook<const N: usize> {
    bids: [MaybeUninit<StackOrder>; N],
    asks: [MaybeUninit<StackOrder>; N],
}

pub struct OrdersTreeInner<const N: usize> {
    inner: [MaybeUninit<StackOrder>; N],
    insert_index: usize,
}

impl<const N: usize> OrdersTreeInner<N> {
    pub fn new() -> OrdersTreeInner<N> {
        OrdersTreeInner {
            inner: [const { MaybeUninit::uninit() }; N],
            insert_index: 0,
        }
    }
}

pub struct StackOrder {
    pub oid: u64,
    pub px: u64,
    pub sz: u64,
    pub next: Option<usize>,
}

impl<const N: usize> StackBook<N> {
    pub fn new() -> StackBook<N> {
        StackBook {
            bids: [const { MaybeUninit::uninit() }; N],
            asks: [const { MaybeUninit::uninit() }; N],
        }
    }

    pub fn place(&mut self, order: StackOrder, bid: bool) {
        for (ix, i) in self.bids.iter().enumerate() {
            if order.px < i.px {}
        }
    }
}
