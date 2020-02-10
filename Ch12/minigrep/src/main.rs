use std::env;
fn main() {
    let thing = env::args().collect();
    println!("{:?}", thing);
}
