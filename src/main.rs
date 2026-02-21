use std::collections::BTreeSet;
use std::io::{self, BufRead};



/*
 * Complete the 'findSmallestMissingPositive' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY orderNumbers as parameter.
 */

pub fn findSmallestMissingPositive(orderNumbers: &[i32]) -> i32 {
	if orderNumbers.len() == 0 {
		return 1;
	}

	use std::collections::BTreeSet;

	let mut set = BTreeSet::from_iter(orderNumbers.iter());
	let mut set = BTreeSet::from_iter(set.iter().filter(|x| x > &&&0));

	drop(orderNumbers);
	//print!("{:?}", set);

	let mut index = 1;
	for value in set {
		//println!("index {}", &index);
		//println!("value {}", &value);
		if value != &&index {
			return index;
		}
		index += 1;
	}
	return index;}

#[cfg(test)]
mod tests {
	use super::*;
	pub struct TestData {
		input_data: [i32; 4],
		expected_result: u8,
	}


	#[test]
	fn findSmallestInt_batch_test()	{
		let a = [23,45,67];

		let td = TestData {
			input_data: [1,2,3,5],
			expected_result: 4,
		};

		println!("{:?}", &td.input_data);
		assert_eq!(findSmallestMissingPositive(&[1,2,3,5]), td.expected_result.into());

		//assert_eq!(findSmallestMissingPositive(&td.input_data), td.expected_result.into());

	}

	#[test]
	fn findSmallestMissingPositive_test() {
		assert_eq!(findSmallestMissingPositive(&[]), 1);
		assert_eq!(findSmallestMissingPositive(&[1]), 2);
		assert_eq!(findSmallestMissingPositive(&[0]), 1);
		assert_eq!(findSmallestMissingPositive(&[-1, -3, -4, -9, -7]), 1);
		assert_eq!(findSmallestMissingPositive(&[1, 3, 4, -9, 7]), 2);
	}


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
