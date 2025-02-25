use std::mem::MaybeUninit;

pub struct StackBook<const N: usize> {
    bids: [MaybeUninit<StackOrder>; N],
    asks: [MaybeUninit<StackOrder>; N],
}

pub struct StackOrder {
    pub oid: u64,
    pub px: u64,
    pub sz: u64,
}

impl<const N: usize> StackBook<N> {
    pub fn new() -> StackBook<N> {
        StackBook {
            bids: [const { MaybeUninit::uninit() }; N],
            asks: [const { MaybeUninit::uninit() }; N],
        }
    }
}
