
extern crate openssl;

use openssl::symm::*;


fn des_example() {
    let data: [u8; 8] = [0; 8];
    let key: [u8; 8] = [0; 8];

    let ciphertext = match encrypt(Cipher::des_ecb(), &key, None, &data) {
        Ok(result) => result,
        Err(err) => {
            println!("Error:{}", err);
            return;
        }
    };

    let plaintext = match decrypt(Cipher::des_ecb(), &key, None, &ciphertext) {
        Ok(result) => result,
        Err(err) => {
            println!("Error:{}", err);
            return;
        }
    };

    println!("plaintext:{:?}", plaintext);
}

fn main() {
    des_example();
}
