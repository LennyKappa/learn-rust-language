use std::cmp::max;
#[allow(dead_code)]

fn largest_index<T : PartialOrd>(list: &[T]) -> &T{

	let mut largest = &list[0];

	for item in list.iter(){
		if item > largest{
			largest = &item;
		}
	}
	largest
}

fn break_down(list: &mut [i32]) -> i32{
	print!("List contents: ");

	for item in list.iter(){
		print!("{} ",item);
	}

	print!("\n");
	if  list.len() == 1 {
		list
	}

	let left = break_down(&list[..list.len()/2]);
	let right = break_down(&list[list.len()/2..]);

	let left_count = 0;
	let right_count = 0;

	let merged = Vec::new();
	while left_count < left.len() && right_count < right.len(){
		match left[left_count].cmp(&right[right.count]) {
			Ordering::Less => {
				merged.push(left[left_count]);
				left_count++;
			}
			_ => {
				merged.push(right[right_count]);
				right_count++;
			}
		}
	}

	if(left_count < left.len()){

	}

	Vec::new()


}
fn main() {
    let thing = vec![1,85,214,467,325,62,67,334,32];
    break_down(&thing);
}
