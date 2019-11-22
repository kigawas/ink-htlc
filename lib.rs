#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::new_ret_no_self)]

use ink_core::env2::call::FromAccountId;
use ink_core::{memory::vec::Vec, storage};
use ink_lang2 as ink;
use sha2::{Digest, Sha256};

use mintable::Mintable;

type Bytes = [u8; 32];

#[ink::contract(version = "0.1.0")]
mod htlc {
    #[ink(storage)]
    struct Htlc {
        token: storage::Value<Mintable>,
        amount: storage::Value<Balance>,
        buyer: storage::Value<AccountId>,
        seller: storage::Value<AccountId>,
        expiration_in_ms: storage::Value<Moment>,
        secret_hash: storage::Value<Bytes>,
    }

    impl Htlc {
        #[ink(constructor)]
        fn new(
            &mut self,
            token_account: AccountId,
            amount: Balance,
            buyer: AccountId,
            expiration_in_ms: Moment,
            secret_hash: Bytes,
        ) {
            self.token.set(Mintable::from_account_id(token_account));

            self.buyer.set(buyer);
            self.seller.set(token_account);

            self.amount.set(amount);

            let now = self.env().now_in_ms();
            assert!(expiration_in_ms > now);
            self.expiration_in_ms.set(expiration_in_ms);

            self.secret_hash.set(secret_hash);
        }

        // read
        #[ink(message)]
        fn balance(&self) -> Balance {
            let this = self.env().address();
            self.token.balance_of(this)
        }

        #[ink(message)]
        fn buyer(&self) -> AccountId {
            *self.buyer
        }

        #[ink(message)]
        fn seller(&self) -> AccountId {
            *self.seller
        }

        #[ink(message)]
        fn expiration_in_ms(&self) -> Moment {
            *self.expiration_in_ms
        }

        #[ink(message)]
        fn secret_hash(&self) -> Bytes {
            *self.secret_hash
        }

        #[ink(message)]
        fn token(&self) -> AccountId {
            self.token.account_id()
        }

        #[ink(message)]
        fn amount(&self) -> Balance {
            *self.amount
        }

        #[ink(message)]
        fn is_valid(&self) -> bool {
            self._is_valid()
        }

        #[ink(message)]
        fn test_sha256(&self, input: Vec<u8>) -> Bytes {
            self._sha256(&input)
        }

        // write

        #[ink(message)]
        fn claim(&mut self, secret: Vec<u8>) -> bool {
            assert!(self._is_valid(), "no enough balance");

            // check timestamp
            let now = self.env().now_in_ms();
            assert!(now <= *self.expiration_in_ms);

            // check sha256(secret) == secret_hash
            assert_eq!(self._sha256(&secret), *self.secret_hash);

            // transfer contract's amount to buyer
            let buyer = *self.buyer;
            let amount = *self.amount;
            self.token.transfer(buyer, amount);

            // TODO: suicide
            true
        }

        #[ink(message)]
        fn refund(&mut self) -> bool {
            // check timestamp
            let now = self.env().now_in_ms();
            assert!(now > *self.expiration_in_ms);

            // burn contract's token
            let amount = *self.amount;
            self.token.burn(amount);

            // TODO: suicide
            true
        }

        fn _is_valid(&self) -> bool {
            let this = self.env().address();
            self.token.balance_of(this) == *self.amount
        }

        fn _sha256(&self, input: &[u8]) -> Bytes {
            let mut hasher = Sha256::new();

            hasher.input(input);

            let result = hasher.result();
            let mut output = [0u8; 32];
            output.copy_from_slice(&result);
            output
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            // let mut token = Mintable::new(String::from("BTC"));
            // token.mint(AccountId::default(), 1);
            // let mut htlc = Htlc::new(token);
        }
    }
}
