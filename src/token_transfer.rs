use crate::GameBloc;
use crate::GameBlocExt;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise};

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payment {
    pub account_id: AccountId,
    pub total_amount: U128,
}

#[near_bindgen]
impl GameBloc {
    #[payable] // Public - People can attach money
    pub fn pay(&mut self, beneficiary: AccountId) -> U128 {
        // Get who is calling the method and how much $NEAR they attached
        let payer: AccountId = env::predecessor_account_id();
        let payable_amount: Balance = env::attached_deposit();

        let mut payed_so_far = self.payments.get(&payer).unwrap_or(0);

        let to_transfer: Balance = if payed_so_far == 0 {
            // This is the user's first donation, lets register it, which increases storage
            assert!(
                payable_amount > STORAGE_COST,
                "Attach at least {} yoctoNEAR",
                STORAGE_COST
            );

            // Subtract the storage cost to the amount to transfer
            payable_amount - STORAGE_COST
        } else {
            payable_amount
        };

        // Persist in storage the amount donated so far
        payed_so_far += payable_amount;
        self.payments.insert(&payer, &payed_so_far);

        log!(
            "Thank you {} for donating {}! You donated a total of {}",
            payer.clone(),
            payable_amount,
            payed_so_far
        );

        // Send the money to the beneficiary
        Promise::new(beneficiary.clone()).transfer(to_transfer);

        // Return the total amount donated so far
        U128(payed_so_far)
    }

    // Public - get donation by account ID
    pub fn get_payments_for_account(&self, account_id: AccountId) -> Payment {
        Payment {
            account_id: account_id.clone(),
            total_amount: U128(self.payments.get(&account_id).unwrap_or(0)),
        }
    }

    // Public - get total number of donors
    pub fn number_of_payments(&self) -> u64 {
        self.payments.len()
    }

    // Public - paginate through all donations on the contract
    pub fn get_payments(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Payment> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through donation
        self.payments
            .keys()
            //skip to the index we specified in the start variable
            .skip(start as usize)
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize)
            .map(|account| self.get_payments_for_account(account))
            //since we turned map into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}
