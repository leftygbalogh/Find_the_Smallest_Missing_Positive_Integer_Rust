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

	// let mut set = BTreeSet::from_iter(orderNumbers.iter());
	// let mut set = BTreeSet::from_iter(set.iter().filter(|x| x > &&&0));
	let mut btset = orderNumbers.iter().collect::<BTreeSet<_>>();
	let mut set = btset.iter().filter(|x| x > &&&0);

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
		let td1 = TestData {
			input_data: [1,2,3,5],
			expected_result: 4,
		};

		let td2 = TestData {
			input_data: [1,2,3,0],
			expected_result: 4,
		};

		let td3 = TestData {
			input_data: [1,2,3,-5],
			expected_result: 4,
		};

		let tdlist = [td1, td2, td3];

		for td in tdlist.iter() {
			println!("{:?}", &td.input_data);
			assert_eq!(findSmallestMissingPositive(&[1,2,3,5]), td.expected_result.into());
		}
	}

	#[test]
	fn findSmallestMissingPositive_test() {
		assert_eq!(findSmallestMissingPositive(&[]), 1);
		assert_eq!(findSmallestMissingPositive(&[1]), 2);
		assert_eq!(findSmallestMissingPositive(&[0]), 1);
		assert_eq!(findSmallestMissingPositive(&[-1, -3, -4, -9, -7]), 1);
		assert_eq!(findSmallestMissingPositive(&[1, 3, 4, -9, 7]), 2);
	}

	#[test]
	fn OneHundredKItems_batch_test()	{
		use rand::prelude::*;

		pub const ONEK: i32 = 100_000;
		#[derive(Debug)]
		pub struct OneHundredKTestData {
			input_data: [i32; ONEK as usize],
			expected_result: u8,
		}

		impl OneHundredKTestData {
			pub fn new_random() -> OneHundredKTestData {
				let mut data = [0;ONEK as usize];
				for _ in 0..ONEK  {
					data.fill(rand::random());
				}
				data[0] = 5;
				data[1] = 3;
				data[2] = 2;
				data[3] = 1;

				for value in data.iter_mut() {
					if *value == 4 {
						*value = 5;
					}
				}


				OneHundredKTestData{input_data: data, expected_result: 4} }
			}

		let data = OneHundredKTestData::new_random();
		//println!("DIPLODOCUS{:?}", &data);
		assert_eq!(findSmallestMissingPositive(&data.input_data), data.expected_result.into())
		}



	}




fn main() {

	println!("\x1b[2J\x1b[H\x1b[3J");
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();

	let orderNumbers_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

	let mut order_numbers: Vec<i32> = Vec::with_capacity(orderNumbers_count as usize);

	for _ in 0..orderNumbers_count {
		let order_numbers_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
		order_numbers.push(order_numbers_item);
	}

	let result = findSmallestMissingPositive(&order_numbers);

	println!("{}", result);
}

//TODO
// Would be nice to handle large inputs as well. See:
// 999
// -5
// -4
// -3
// -2
// -1
// 0
// 1
// 2
// 4
// 6303603237
//
// thread 'main' (16512) panicked at src\main.rs:137:96:
// called `Result::unwrap()` on an `Err` value: ParseIntError { kind: PosOverflow }
// stack backtrace:
