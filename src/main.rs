use std::collections::{BTreeSet, HashSet};
use std::io::{self, BufRead};



/*
 * Complete the 'findSmallestMissingPositive' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY orderNumbers as parameter.
 */

fn findSmallestMissingPositive(orderNumbers: &[i32]) -> i32 {

	// if orderNumbers.len() == 0 {
	// 	return 1;
	// }

	// Write your code here
	println!("SmallestMissingPositive orderNumbers: {:?}", &orderNumbers);
	let mut set: BTreeSet<&i32> = BTreeSet::new();
	for num in orderNumbers {
		set.insert(&num);
	}
	println!("{:?}", &set);

	//let l1 = set.iter().filter(|p| !set.contains(p) );
	set.retain(|x| {x > &&0 });

	let mut length = set.len();
	let mut l2: i32 = length.try_into().unwrap();
		set.retain(|x| {x < &&l2   });

	println!("{:?}", &set);

	let mut length = set.len();
	let mut l2: i32 = length.try_into().unwrap();
	l2+1
}

fn main() {

	println!("\x1b[2J\x1b[H\x1b[3J");
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
