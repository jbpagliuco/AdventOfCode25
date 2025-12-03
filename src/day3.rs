use crate::Solver;

#[allow(dead_code)]
pub const FILENAME: &str = file!();

struct Bank {
	batteries: Vec<u32>,
}

// Part 1
pub struct Part1 {
	banks: Vec<Bank>,
}

impl Solver for Part1 {
	type Output = u64;

	fn parse(input: String) -> Self {
		Self {
			banks: input
				.lines()
				.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
				.map(|batteries| Bank {
					batteries,
				})
				.collect::<Vec<Bank>>(),
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(16858)
	}

	fn solve(&self) -> Option<Self::Output> {
		let sum = self.banks.iter().map(|bank| max_joltage(&bank.batteries, 2)).sum();
		Some(sum)
	}
}



// Part 2
pub struct Part2 {
	banks: Vec<Bank>,
}

impl Solver for Part2 {
	type Output = u64;

	fn parse(input: String) -> Self {
		Self {
			banks: input
				.lines()
				.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
				.map(|batteries| Bank {
					batteries,
				})
				.collect::<Vec<Bank>>(),
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(167549941654721)
	}

	fn solve(&self) -> Option<Self::Output> {
		let sum = self.banks.iter().map(|bank| max_joltage(&bank.batteries, 12)).sum();
		Some(sum)
	}
}



/// Helpers

fn max_index(values: &[u32]) -> (usize, u32) {
	let mut max: (usize, u32) = (0, values[0]);

	for (i, &value) in values.iter().enumerate() {
		if value > max.1 {
			max = (i, value);
		}
	}

	max
}

fn max_joltage(batteries: &Vec<u32>, num_batteries: usize) -> u64 {
	let mut joltage = 0u64;

	let mut i_prev = None;
	for i_battery in 0..num_batteries {
		let i_start = i_prev.map(|i| i + 1).unwrap_or(0);
		let i_last = batteries.len() - num_batteries + i_battery + 1;

		let max = max_index(&batteries[i_start..i_last]);
		i_prev = Some(max.0 + i_start);

		joltage = joltage * 10 + max.1 as u64;
	}

	joltage
}
