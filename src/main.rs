mod math{
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    println!("Addition: {}", math::add(10, 20));
    println!("Subtraction: {}", math::sub(20, 10));
}
