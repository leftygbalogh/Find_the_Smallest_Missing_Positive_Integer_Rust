use std::collections::HashSet;
use std::io::{self, BufRead};



/*
 * Complete the 'findSmallestMissingPositive' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY orderNumbers as parameter.
 */

fn findSmallestMissingPositive(orderNumbers: &[i32]) -> i32 {
	// Write your code here
	println!("SmallestMissingPositive orderNumbers: {:?}", &orderNumbers);
	let mut set: HashSet<&i32> = HashSet::new();
	for num in orderNumbers {
		set.insert(&num);
	}
	println!("{:?}", set);
	42

}

fn main() {
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();

	let orderNumbers_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

	let mut orderNumbers: Vec<i32> = Vec::with_capacity(orderNumbers_count as usize);

	for _ in 0..orderNumbers_count {
		let orderNumbers_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
		orderNumbers.push(orderNumbers_item);
	}

	let result = findSmallestMissingPositive(&orderNumbers);

	println!("{}", result);
}
