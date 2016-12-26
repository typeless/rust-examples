extern crate openssl;

use self::openssl::symm::*;

pub fn des_example() {
    let data: [u8; 8] = [11, 12, 13, 14, 15, 16, 17, 18];
    let key: [u8; 8] = [21, 22, 23, 24, 25, 26, 27, 28];

    let ciphertext = match encrypt(Cipher::des_ecb(), &key, None, &data) {
        Ok(result) => result,
        Err(err) => {
            println!("Error:{}", err);
            return;
        }
    };
    println!("ciphertext:{:?}", ciphertext);

    let plaintext = match decrypt(Cipher::des_ecb(), &key, None, &ciphertext) {
        Ok(result) => result,
        Err(err) => {
            println!("Error:{}", err);
            return;
        }
    };

    println!("plaintext:{:?}", plaintext);
}
