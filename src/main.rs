use std::io::{self, BufRead};

/*
 * Complete the 'findSmallestMissingPositive' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY orderNumbers as parameter.
 */

pub fn find_smallest_missing_positive(order_numbers: &[i64]) -> i64 {
    if order_numbers.is_empty() {
        return 1;
    }

    use std::collections::BTreeSet;

    // let mut set = BTreeSet::from_iter(orderNumbers.iter());
    // let mut set = BTreeSet::from_iter(set.iter().filter(|x| x > &&&0));
    let btset = order_numbers.iter().collect::<BTreeSet<_>>();
    let set = btset.iter().filter(|x| x > &&&0);

    let _ = order_numbers;
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
    index
}

#[cfg(test)]
mod tests {
	use super::*;
	pub struct TestData {
        input_data: [i64; 4],
        expected_result: u8,
    }

    #[test]
    fn findSmallestInt_batch_test() {
        let td1 = TestData {
            input_data: [1, 2, 3, 5],
            expected_result: 4,
        };

        let td2 = TestData {
            input_data: [1, 2, 3, 0],
            expected_result: 4,
        };

        let td3 = TestData {
            input_data: [1, 2, 3, -5],
            expected_result: 4,
        };

        let tdlist = [td1, td2, td3];

        for td in tdlist.iter() {
            println!("{:?}", &td.input_data);
            assert_eq!(
				find_smallest_missing_positive(&[1, 2, 3, 5]),
				td.expected_result.into()
            );
        }
    }

    #[test]
    fn find_Smallest_Missing_Positive_test() {
        assert_eq!(find_smallest_missing_positive(&[]), 1);
        assert_eq!(find_smallest_missing_positive(&[1]), 2);
        assert_eq!(find_smallest_missing_positive(&[0]), 1);
        assert_eq!(find_smallest_missing_positive(&[-1, -3, -4, -9, -7]), 1);
        assert_eq!(find_smallest_missing_positive(&[1, 3, 4, -9, 7]), 2);
        assert_eq!(
			find_smallest_missing_positive(
                 &[
                    9644455085, 2397179666, 6886896129, 6935504988, 6903521541, 6745404418,
                    4539618744, 5258576932, 1685165598, 8761923581, 9480182470, 4633157410,
                    6547702252, 8277071998, 6329088993, 4408175415, 1824213094, 1377795507,
                    7691655004, 6563316458, 5273844144, 4807701266, 7476734799, 9793300143,
                    8866728132, 6470187763, 8928181836, 6041566302, 8247724479, 7983418739,
                    3817079407, 5479497660, 5550151131, 9963439898, 7134367162, 1045464330,
                    3137907629
                ]
            ),
			1
        );
    }

    #[test]
    fn One_Hundred_K_Items_batch_test() {
        pub const ONEK: i64 = 85_555;
        #[derive(Debug)]
        pub struct OneHundredKTestData {
            input_data: [i64; ONEK as usize],
            expected_result: u8,
        }

        impl OneHundredKTestData {
            pub fn new_random() -> OneHundredKTestData {
                let mut data = [0; ONEK as usize];
                for _ in 0..ONEK {
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

                OneHundredKTestData {
                    input_data: data,
                    expected_result: 4,
                }
            }
        }

        let data = OneHundredKTestData::new_random();
        //println!("DIPLODOCUS{:?}", &data);
        assert_eq!(
			find_smallest_missing_positive(&data.input_data),
			data.expected_result.into()
        )
    }
}

fn main() {
    println!("\x1b[2J\x1b[H\x1b[3J");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let order_numbers_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();

    let mut order_numbers: Vec<i64> = Vec::with_capacity(order_numbers_count as usize);

    for _ in 0..order_numbers_count {
        let order_numbers_item = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap();
        order_numbers.push(order_numbers_item);
    }

    let result = find_smallest_missing_positive(&order_numbers);

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
