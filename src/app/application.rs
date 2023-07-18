use std::{collections::HashMap, error::Error};

use crate::CONSENSUS_ADDRESS;

use super::{transaction::Transaction, address::Address, account::Account, receipt::{Receipt, Status}};

impl Transaction {

   pub fn application(

       &self,
       changed_accounts: &mut HashMap<Address, Account>,
       mut recipient: Account,
       mut sender: Account
       
   ) -> Result<Receipt, Box<dyn Error>> {

        if sender.counter == self.counter {

            let mut solar_used = 0;

            let transaction_cost = opis::Integer::from_dec("1000")?;

            sender.decrease_balance(&transaction_cost)?;

            solar_used += 1000;

            if self.sender != self.recipient {

                if self.recipient == CONSENSUS_ADDRESS {
                    
                    if self.value != opis::Integer::zero() {
                        
                        sender.decrease_balance(&self.value)?;

                        // match Account::from_accounts(&self.sender, &changed_accounts, &accounts_store) {

                        //     Ok(_consensus) => {

                        //         // add stake

                        //         Err("")?

                        //     }

                        //     Err(_) => Err("")?,

                        // }

                        Err("")?

                    } else {

                        // decrease stake by amount in tx data

                        // add value to user account

                        Err("")?

                    }


                } else {

                    if recipient.balance == opis::Integer::zero() && recipient.counter == opis::Integer::zero() {
                        // charge account creation 
                    }

                    match sender.decrease_balance(&self.value) {

                        Ok(_) => {

                            

                            recipient.balance += &self.value;

                            sender.counter += opis::Integer::one();

                            changed_accounts.insert(self.sender, sender);

                            changed_accounts.insert(self.recipient, recipient);

                            Ok(Receipt::new(solar_used, Status::Accepted))

                        },

                        Err(_) => {

                            sender.counter += opis::Integer::one();

                            changed_accounts.insert(self.sender, sender);

                            Ok(Receipt::new(solar_used, Status::BalanceError))

                        },

                    }

                }
            } else {
                Err("")?
            }
        } else {
            Err("")?
        }
   }
}
