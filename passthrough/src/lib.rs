#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Passthrough {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
