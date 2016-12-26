use openssl::rsa::Rsa;
use openssl::pkey::PKey;

pub fn main() {
    let key = match Rsa::generate(32) {
        Err(err) => {
            println!("Error:{:?}", err);
            return;
        }
        Ok(key) => key,
    };

    println!("RSA key:{:#?}", key);
}
