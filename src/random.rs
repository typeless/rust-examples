use rand;

pub fn main() {
    let x = rand::random::<u8>();
    println!("Random x: {}", x);

    let xs = rand::random::<[u8; 8]>();
    println!("Random xs: {:?}", xs);
}
