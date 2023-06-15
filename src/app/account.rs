use std::{collections::{BTreeMap, HashMap}, error::Error};

use super::{address::Address, object::Object, storage::{storage_search, storage_list}};

#[derive(Clone,Debug)]
pub struct Account {
    pub balance: opis::Integer,
    pub counter: opis::Integer,
    pub storage: [u8; 32],
}

impl Account {

    pub fn new() -> Account {

        Account {
            balance: opis::Integer::zero(),
            counter: opis::Integer::zero(),
            storage: [0_u8; 32],
        }

    }

    // pub fn try_from_storage(
    //     object_store: &neutrondb::Store<[u8; 32], Object>,
    //     address: &[u8; 32],
    //     accounts_hash: &[u8; 32],
    // ) -> Result<Account, Box<dyn Error>> {

    //     match storage_search(address, object_store, *accounts_hash)? {
            
    //         Some(account_details) => {

    //             let account_objects = storage_list(
    //                 object_store,
    //                 account_details
    //                     .data
    //                     .as_slice()
    //                     .try_into()
    //                     .map_err(|_| "Invalid account data: Not a valid hash")?,
    //             )?;

    //             if account_objects.len() != 3 {
    //                 return Err("Account field error!")?;
    //             }
        
    //             let balance = opis::Integer::from(account_objects[0].data);
        
    //             let counter = opis::Integer::from(account_objects[1].data);
        
    //             let storage = account_objects[2].hash();
        
    //             Ok(Account {
    //                 balance,
    //                 counter,
    //                 storage,
    //             })

    //         },

    //         None => Err("Not Found!")?,

    //     }
        
    // }

    pub fn from_accounts(
        address: &Address,
        changed_accounts: &HashMap<Address, Account>,
        accounts_store: &neutrondb::Store<Address, Account>
    ) -> Result<Account, Box<dyn Error>> {

        match changed_accounts.get(address) {
            Some(account) => Ok(account.clone()),
            None => { accounts_store.get(address) }
        }

    }

    pub fn increase_counter(&mut self) {
        self.counter += opis::Integer::one()
    }

    pub fn storage_hash(&self) -> [u8; 32] {

        todo!()

    }

    pub fn increase_balance(&mut self, amount: &opis::Integer) {

        self.balance += amount;

    }

    pub fn decrease_balance(&mut self, amount:&opis::Integer) -> Result<(), Box<dyn Error>> {

        if &self.balance >= amount {

            self.balance -= amount;

            Ok(())

        } else {

            Err("Not enough balance!")?

        }
    }

    pub fn details_hash(&self) -> [u8; 32] {

        let balance: Vec<u8> = (&self.balance).into();

        let counter: Vec<u8> = (&self.counter).into();

        fides::merkle_tree::root(
            fides::hash::blake_3,
            &[
                &balance,
                &counter,
                &self.storage
            ]
        )
        
    }

}

impl TryFrom<&[u8]> for Account {

    fn try_from(arg: &[u8]) -> Result<Self, Box<dyn Error>> {

        let account_fields = astro_format::decode(arg)?;

        if account_fields.len() == 3 {

            let mut result = Account {
                balance: opis::Integer::from(account_fields[0]),
                counter: opis::Integer::from(account_fields[1]),
                storage: account_fields[2].try_into()?,
            };

            Ok(result)

        } else {

            Err("Account fields error!")?

        }

    }

    type Error = Box<dyn Error>;

}

impl TryFrom<Vec<u8>> for Account {
    type Error = Box<dyn Error>;
    fn try_from(value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Account::try_from(&value[..])
    }
}