use std::{error::Error, env, path::Path, fs};

use app::address::Address;
mod app;

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

                if !Path::new("./keys").exists() { fs::create_dir("./keys")?; };

                fs::write(format!("./keys/{}.bin", public_key_hex), &secret_key)?;

                println!("Done, Account Address is {}", public_key_hex);

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

new     creates a new account

    "###);

}
