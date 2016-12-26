
extern crate openssl;
extern crate rand;

mod des;
mod random;
mod rsa;

fn main() {
    des::main();
    random::main();
    rsa::main();
}
