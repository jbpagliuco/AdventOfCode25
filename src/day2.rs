use crate::Solver;

#[allow(dead_code)]
pub const FILENAME: &str = file!();

// Part 1
pub struct Part1 {
	ranges: Vec<(String, String)>,
}

impl Solver for Part1 {
	type Output = i64;

	fn parse(input: String) -> Self {
		Self {
			ranges: input
				.split(",")
				.map(|split| {
					let (first, last) = split.split_once("-").unwrap();
					(first.to_owned(), last.to_owned())
				})
				.collect(),
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(12850231731)
	}

	fn solve(&self) -> Option<Self::Output> {
		let mut sum = 0;

		for range in &self.ranges {
			let (first, last) = (range.0.parse::<i64>().unwrap(), range.1.parse::<i64>().unwrap());
			for i in first..=last {
				let number = i.to_string();
				let (half1, half2) = number.split_at(number.len() / 2);
				if half1 == half2 {
					sum += i;
				}
			}
		}

		Some(sum)
	}
}



// Part 2
pub struct Part2 {
	ranges: Vec<(String, String)>,
}

impl Solver for Part2 {
	type Output = i64;

	fn parse(input: String) -> Self {
		Self {
			ranges: input
				.split(",")
				.map(|split| {
					let (first, last) = split.split_once("-").unwrap();
					(first.to_owned(), last.to_owned())
				})
				.collect(),
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(24774350322)
	}

	fn solve(&self) -> Option<Self::Output> {
		let mut sum = 0;

		for range in &self.ranges {
			let (first, last) = (range.0.parse::<i64>().unwrap(), range.1.parse::<i64>().unwrap());
			for number in first..=last {
				let numberstr = number.to_string();
				let len = numberstr.len();

				for i in 1..=len / 2 {
					// Grab substring with i characters
					let substr = &numberstr[0..i];

					// Check if substring repeats throughout the rest of the string
					let repeats = 'check_repeat: {
						for j in (i..len).step_by(i) {
							if j + i > len {
								break 'check_repeat false;
							}

							let cur = &numberstr[j..j + i];
							if substr != cur {
								break 'check_repeat false;
							}
						}

						true
					};

					if repeats {
						sum += number;
						break;
					}
				}
			}
		}

		Some(sum)
	}
}
