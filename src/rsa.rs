use openssl::rsa::Rsa;
use openssl::rsa::NO_PADDING;
use rand;
use itertools::Itertools;

pub fn main() {
    let key = match Rsa::generate(2048) {
        Err(err) => {
            println!("Error:{:?}", err);
            return;
        }
        Ok(key) => key,
    };

    println!("RSA key:{:#?}", key);

    let plaintext = rand::random::<[u8; 32]>();
    let mut ciphertext: [u8; 256] = [0; 256];
    match key.private_encrypt(&plaintext[..], &mut ciphertext, NO_PADDING) {
        Err(err) => println!("Error:{:?}", err),
        Ok(size) => println!("size:{}", size),
    }


    println!("RSA private encrypted: {:02x}",
             ciphertext.iter().format(""));
}
