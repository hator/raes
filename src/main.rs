#[macro_use]
extern crate arrayref;

mod aes;


fn main() {
    // let block = aes::Block::new(&[1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8]);
    // let key = aes::Key::new(&[1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8]);
    let block = aes::Block::new(&[1,5,1,5,2,6,2,6,3,7,3,7,4,8,4,8]);
    let key = aes::Key::new(&[1,5,1,5,2,6,2,6,3,7,3,7,4,8,4,8]);

    let encrypted = aes::encrypt(key, block);

    println!("{:?}", encrypted);
}
