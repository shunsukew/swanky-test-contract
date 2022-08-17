#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod attacker_wallet {
    #[ink(storage)]
    pub struct AttackerWallet {
        owner: AccountId,
        flip: bool,
    }

    impl AttackerWallet {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { owner: Self::env().caller(), flip: false }
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.env().balance()
        }

        #[ink(message)]
        pub fn transfer_to(&mut self, to: AccountId, amount: Balance) {
            assert!(self.env().caller() == self.owner, "only owner can transfer funds");
            assert!(amount <= self.env().balance(), "insufficient funds!");

            if self.env().transfer(to, amount).is_err() {
                panic!("requested transfer failed.")
            }
        }

        #[ink(message)]
        pub fn get_flip(&self) -> bool {
            self.flip
        }

        #[ink(message, payable, selector = _)]
        pub fn fallback(&mut self) {
            self.flip = !self.flip;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let accounts = default_accounts();
            set_sender(accounts.alice);

            let attacker_wallet = AttackerWallet::new();
            assert_eq!(attacker_wallet.get_owner(), accounts.alice);
        }

        #[ink::test]
        fn get_balance_works() {
            // given
            let contract_balance = 100;
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let attacker_wallet = create_contract(contract_balance);

            // then
            assert_eq!(attacker_wallet.get_balance(), 100);
        }

        #[ink::test]
        fn transfer_to_works() {
            // given
            let contract_balance = 100;
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let mut attacker_wallet = create_contract(contract_balance);

            // when
            set_balance(accounts.bob, 0);
            attacker_wallet.transfer_to(accounts.bob, 80);

            // then
            assert_eq!(get_balance(accounts.bob), 80);
        }

        fn contract_id() -> AccountId {
            ink_env::test::callee::<ink_env::DefaultEnvironment>()
        }

        fn default_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(sender);
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(
                account_id, balance,
            )
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(account_id)
                .expect("Cannot get account balance")
        }

        fn create_contract(initial_balance: Balance) -> AttackerWallet {
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), initial_balance);
            AttackerWallet::new()
        }
    }
}
