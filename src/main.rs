use oper::math::add;
use oper::math::sub;
use rand::random;

mod oper;

fn main() {
    println!("Addition: {}", add(10, 20));
    println!("Subtraction: {}", sub(20, 10));
    println!("Random number between 1 and 255: {}", random::<u8>());
}
