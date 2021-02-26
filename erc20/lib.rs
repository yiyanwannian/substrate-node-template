#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct Erc20 {
        issuer: AccountId,
        total_supply: Balance,
        balances: StorageHashMap<AccountId, Balance>,
        allowance: StorageHashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct SetAllowance {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InSufficientBalance,
        InSufficientAllowance,
        IssuePermissionDenied,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = StorageHashMap::new();
            balances.insert(caller, total_supply);
            let instance = Self {
                issuer: caller,
                total_supply: total_supply,
                balances: balances,
                allowance: StorageHashMap::new(),
            };
            instance
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            // todo
            *self.allowance.get(&(owner, spender)).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn set_allowance(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            self.allowance.insert((caller, spender), value);
            self.env().emit_event(SetAllowance {
                owner: caller,
                spender: spender,
                value: value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            // todo
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller); 
            if allowance < value {
                return Err(Error::InSufficientAllowance);
            }
            match self.transfer_help(from, to, value) {
                Ok(_) => {
                    self.allowance.insert((from, caller), allowance - value);
                    Ok(())
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        #[ink(message)]
        pub fn burn(&mut self, value: Balance) -> Result<()> {
            // todo
            let who = self.env().caller();
            let from_balance = self.balance_of(who);
            if from_balance < value {
                return Err(Error::InSufficientBalance);
            }
            self.balances.insert(who, from_balance - value);
            let total = self.total_supply();
            self.total_supply = total - value;

            Ok(())
        }

        #[ink(message)]
        pub fn issue(&mut self, value:Balance) -> Result<()> {
            // todo
            let who = self.env().caller();
            if who != self.issuer{
                return Err(Error::IssuePermissionDenied)
            }
            let total = self.total_supply();
            self.total_supply = total + value;
            let balance = self.balance_of(who);
            self.balances.insert(who, balance + value);

            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let who = self.env().caller();
            self.transfer_help(who, to, value)
        }

        fn transfer_help(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InSufficientBalance);
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);

            self.env().emit_event(Transfer {
                from: from,
                to: to,
                value: value,
            });
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink_lang::test]
        fn create_contract_works() {
            let erc20 = Erc20::new(1000);
            assert_eq!(erc20.total_supply(), 1000);
        }
    }
}
