use std::{error::Error, env, path::Path, fs};

use app::address::Address;

use crate::app::{App, chain::ChainID};
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
                    let _ = app.sync()?;
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
                    let _ = app.sync()?;
                    let _ = app.mine(&account_key)?;
                    loop {}
                } else {
                    Err("Use mine [chain id] [address]")?
                }
            },

            "send" => {

            },

            "stake" => {

            },

            "withdraw" => {

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

new ................................................... creates a new account
sync [chain] .......................................... validates the blockchain
mine [chain] [address] ................................ extends the blockchain
stake [chain] [address] [value] ....................... adds stake
withdraw [chain] [address] [value] .................... removes stake
send [value] on [chain] from [address] to [address] ... send solar

    "###);

}
