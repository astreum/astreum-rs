use std::{collections::{BTreeMap, HashMap}, error::Error};

use super::address::Address;

#[derive(Clone,Debug)]
pub struct Account {
    pub balance: opis::Integer,
    pub counter: opis::Integer,
    pub details_hash: [u8; 32],
    pub storage: BTreeMap<Vec<u8>, Vec<u8>>,
    pub storage_hash: [u8; 32],
}

impl Account {

    pub fn new() -> Account {

        Account {
            balance: opis::Integer::zero(),
            counter: opis::Integer::zero(),
            storage: BTreeMap::new(),
            storage_hash: [0_u8; 32],
            details_hash: [0_u8; 32],
        }

    }

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

        let storage = self.storage
            .iter()
            .map(|x| [fides::hash::blake_3(x.0), fides::hash::blake_3(x.1)].concat())
            .collect::<Vec<_>>();
        
        fides::merkle_tree::root(
            fides::hash::blake_3,
            &(storage
                .iter()
                .map(|x| x.as_slice())
                .collect::<Vec<_>>()
            )
        )

    }

    pub fn update_storage_hash(&mut self) {
        self.storage_hash = self.storage_hash()
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
                &self.storage_hash
            ]
        )
        
    }

}

impl TryFrom<&[u8]> for Account {

    fn try_from(arg: &[u8]) -> Result<Self, Box<dyn Error>> {

        let account_details = astro_format::decode(arg)?;

        if account_details.len() == 3 {

            let decoded_storage = astro_format::decode(account_details[2])?;

            let mut storage = BTreeMap::new();

            for i in decoded_storage {        

                let decoded_kv = astro_format::decode(i)?;

                if decoded_kv.len() == 2 {

                    storage.insert(
                        decoded_kv[0].try_into()?,
                        decoded_kv[1].try_into()?
                    );

                }

            }

            let mut result = Account {
                balance: opis::Integer::from(account_details[0]),
                counter: opis::Integer::from(account_details[1]),
                details_hash: [0_u8; 32],
                storage: storage,
                storage_hash: [0_u8; 32]
            };

            result.update_storage_hash();

            Ok(result)

        } else {

            Err("Internal error!")?

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