
extern crate openssl;
extern crate rand;
extern crate itertools;

mod des;
mod random;
mod rsa;

fn main() {
    des::main();
    random::main();
    rsa::main();
}
