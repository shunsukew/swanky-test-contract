#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod attacker {
    #[ink(storage)]
    pub struct Attacker {
        owner: AccountId,
        flip: bool,
    }

    impl Attacker {
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
        pub fn get_flip(&self) -> bool {
            self.flip
        }

        #[ink(message, payable, selector = _)]
        pub fn fallback(&self) {
            ink_env::debug_println!("Fallback message called");
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

            let attacker = Attacker::new();
            assert_eq!(attacker.get_owner(), accounts.alice);
        }

        #[ink::test]
        fn get_balance_works() {
            // given
            let contract_balance = 100;
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let attacker = create_contract(contract_balance);

            // then
            assert_eq!(attacker.get_balance(), 100);
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

        fn create_contract(initial_balance: Balance) -> Attacker {
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), initial_balance);
            Attacker::new()
        }
    }
}
