use crate::Solver;

#[allow(dead_code)]
pub const FILENAME: &str = file!();

// Part 1
pub struct Part1 {
	deltas: Vec<i32>,
}

impl Solver for Part1 {
	type Output = i32;

	fn parse(input: String) -> Self {
		Self {
			deltas: input
				.lines()
				.map(|input| {
					let (direction, delta) = input.split_at(1);
					delta.parse::<i32>().unwrap() * if direction == "L" { -1 } else { 1 }
				})
				.collect::<Vec<i32>>(),
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(1180)
	}

	fn solve(&self) -> Option<Self::Output> {
		const MAX: i32 = 100;
		let mut zeros = 0;
		let mut dial = 50i32;
		for delta in &self.deltas {
			dial = (dial + delta).rem_euclid(MAX);

			if dial == 0 {
				zeros += 1;
			}
		}

		Some(zeros)
	}
}



// Part 2
pub struct Part2 {
	deltas: Vec<i32>,
}

impl Solver for Part2 {
	type Output = i32;

	fn parse(input: String) -> Self {
		Self {
			deltas: input
				.lines()
				.map(|input| {
					let (direction, delta) = input.split_at(1);
					delta.parse::<i32>().unwrap() * if direction == "L" { -1 } else { 1 }
				})
				.collect::<Vec<i32>>(),
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(6892)
	}

	fn solve(&self) -> Option<Self::Output> {
		const MAX: i32 = 100;

		let mut zeros = 0i32;
		let mut dial = 50i32;
		for delta in &self.deltas {
			let next = dial + delta;

			if next <= 0 && dial != 0 {
				zeros += 1;
			}

			zeros += next.abs() / MAX;

			dial = next.rem_euclid(MAX);
		}

		Some(zeros)
	}
}
