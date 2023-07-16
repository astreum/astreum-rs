use std::collections::HashMap;
use std::error::Error;
use crate::storage::Storage;
use super::address::Address;

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

    pub fn storage_hash(&self) -> [u8; 32] {
        todo!()
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

impl Storage {

    pub fn get_account(&self, accounts: &[u8;32], address: &Address) -> Result<Account, Box<dyn Error>> {
        let detail_root_object = self.search_object(&address.0, accounts)?;
        let detail_objects = self.get_list(&detail_root_object.hash())?;
        if detail_objects.len() != 3 {
            return Err("Account field error!")?;
        }
        let balance = opis::Integer::from(&detail_objects[0].data[..]);
        let counter = opis::Integer::from(&detail_objects[1].data[..]);
        let storage = detail_objects[2].hash();
        Ok(Account {
            balance,
            counter,
            storage,
        })
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