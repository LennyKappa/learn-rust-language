fn main() {
    let v = vec![1,2,3,4,5];
    match v.get(2) {
    	Some(x) => println!("v[2] = {}", x),
    	None => println!("v[2] does not exist."),
    }
    match v.get(100) {
    	Some(x) => println!("v[100] = {}", x),
    	None => println!("v[100] does not exist."),
    }
}
