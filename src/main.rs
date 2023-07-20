use std::{error::Error, env, path::Path, fs};

use app::address::Address;

use crate::{app::{App, chain::ChainID, transaction::Transaction}, relay::{message::Message, topic::Topic, route::RouteID}};
mod app;
mod relay;
mod storage;

const CONSENSUS_ADDRESS: Address = Address([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 111, 110, 115, 101, 110, 115, 117, 115]);
const STELAR_ADDRESS: Address = Address([0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 111, 110, 115, 101, 110, 115, 117, 115]);


fn main() -> Result<(), Box<dyn Error>> {

    println!("Astreum Rust v0.1.0");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {

        let topic : &str = &args[1];

        match topic {

            "new" => {
                println!("Creating Account ...");
                let secret_key = fides::ed25519::secret_key();
                let public_key = fides::ed25519::public_key(&secret_key)?;
                let public_key_hex = hex::encode(&public_key);
                if !Path::new("./keys").exists() {
                    fs::create_dir("./keys")?;
                };
                fs::write(format!("./keys/{}.bin", public_key_hex), &secret_key)?;
                println!("Done, Account Address is {}", public_key_hex);

            },

            "sync" => {
                println!("Syncing Blockchain ...");
                if args.len() == 3 {
                    let chain_id = ChainID::try_from(&args[2][..])?;
                    let app = App::new(chain_id, false)?;
                    app.sync()?;
                    app.validate()?;
                    loop {}
                } else {
                    Err("Use sync [chain id]")?
                }
            },

            "mine" => {
                println!("Mining ...");
                if args.len() == 4 {
                    let account_key_path_str = format!("./keys/{:?}", &args[3]);
                    let account_key_path = Path::new(&account_key_path_str);
                    let account_key: [u8;32] = fs::read(account_key_path)?[..].try_into()?;
                    let chain_id = ChainID::try_from(&args[2][..])?;
                    let app = App::new(chain_id, true)?;
                    app.sync()?;
                    app.validate()?;
                    app.mine(account_key)?;
                    loop {}
                } else {
                    Err("Use mine [chain id] [address]")?
                }
            },

            "send" => {
                println!("Sending Solar ...");
                if args.len() == 9 {
                    let chain_id = ChainID::try_from(&args[4][..])?;
                    let app = App::new(chain_id.clone(), true)?;
                    app.sync()?;
                    // connect
                    let account_key_path_str = format!("./keys/{:?}", &args[6]);
                    let account_key_path = Path::new(&account_key_path_str);
                    let account_key: [u8;32] = fs::read(account_key_path)?[..].try_into()?;
                    let sender: Address = args[6][..].try_into()?;
                    let recipient: Address = args[9][..].try_into()?;
                    let accounts = match app.latest_block_pointer.lock() {
                        Ok(latest_block) => latest_block.accounts,
                        Err(_) => Err("Latest Block Pointer Error!")?,
                    };
                    let counter = match app.storage_pointer.lock() {
                        Ok(storage) => match storage.get_account(&accounts, &sender) {
                            Ok(account) => account.counter,
                            Err(_) => Err("Get Account Error!")?,
                        },
                        Err(_) => Err("Storage Pointer Error!")?,
                    };
                    let mut tx = Transaction {
                        chain_id,
                        counter,
                        data: Vec::new(),
                        details_hash: [0_u8;32],
                        hash: [0_u8;32],
                        recipient,
                        sender,
                        signature: [0_u8;64],
                        value: opis::Integer::from_dec(&args[2])?,
                    };

                    tx.details_hash();
                    tx.signature(&account_key)?;
                    tx.hash();

                    let tx_msg = Message {
                        body: tx.hash.to_vec(),
                        topic: Topic::Transaction,
                    };

                    match app.relay_pointer.clone().lock() {
                        Ok(relay) => {
                            relay.broadcast(RouteID::Consensus, tx_msg)?;
                        },
                        Err(_) => Err("Relay Pointer Error!")?,
                    }

                    // check execution
                    
                }
            },

            "withdraw" => {
                println!("Withdraw ...");
            },

            _ => help()

        }

    } else {
        
        help()

    }

    Ok(())
    
}

fn help() {

    println!(r###"
Help
- - - + - - - + - - -

new ................................................... create account
sync [chain] .......................................... check blocks
mine [chain] [address] ................................ create blocks
send [value] on [chain] from [address] to [address] ... send solar
withdraw [chain] [address] [value] .................... removes stake


    "###);

}
