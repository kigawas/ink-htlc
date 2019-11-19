#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::new_ret_no_self)]

use ink_core::storage;
use ink_lang2 as ink;

use mintable::Mintable;

#[ink::contract(version = "0.1.0")]
mod htlc {
    #[ink(storage)]
    struct Htlc {
        token: storage::Value<Mintable>,
    }

    impl Htlc {
        #[ink(constructor)]
        fn new(&mut self, token: Mintable) {
            self.token.set(token);
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        fn get(&self) -> Balance {
            self.token.total_supply()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let mut token = Mintable::new(String::from("BTC"));
            token.mint(AccountId::default(), 1);
            // let mut htlc = Htlc::new(token);
        }
    }
}
