use crate::Solver;

#[allow(dead_code)]
pub const FILENAME: &str = file!();

enum Operator {
	Plus,
	Multiply,
}

// Part 1
pub struct Part1 {
	operands: Vec<Vec<u64>>,
	operators: Vec<Operator>,
}

impl Solver for Part1 {
	type Output = u64;

	fn parse(input: String) -> Self {
		let mut lines: Vec<_> = input.lines().collect();

		let operators = lines
			.pop()
			.unwrap()
			.split_whitespace()
			.map(|s| if s == "+" { Operator::Plus } else { Operator::Multiply })
			.collect();

		let mut operands = vec![];
		for line in lines {
			operands.push(line.split_whitespace().map(|s| s.parse().unwrap()).collect());
		}

		Self {
			operands,
			operators,
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(4722948564882)
	}

	fn solve(&self) -> Option<Self::Output> {
		let mut answers = Vec::with_capacity(self.operators.len());
		for (i, operator) in self.operators.iter().enumerate() {
			let operands = self.operands.iter().map(|operands| operands[i]);
			let answer = operands.fold(
				match operator {
					Operator::Plus => 0,
					Operator::Multiply => 1,
				},
				|acc, operand| match operator {
					Operator::Plus => acc + operand,
					Operator::Multiply => acc * operand,
				},
			);
			answers.push(answer);
		}

		Some(answers.iter().sum())
	}
}



// Part 2
pub struct Part2 {
	operands: Vec<Vec<char>>,
	operators: Vec<char>,
}

impl Solver for Part2 {
	type Output = u64;

	fn parse(input: String) -> Self {
		let mut lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

		let operators = lines.pop().unwrap();
		let operands = lines;

		Self {
			operands,
			operators,
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(9581313737063)
	}

	fn solve(&self) -> Option<Self::Output> {
		struct Acc {
			total: u64,
			cur: u64,
			cur_operator: char,
		}

		let rows = self.operands.len();

		let result = self.operators.iter().enumerate().fold(
			Acc {
				total: 0,
				cur: 0,
				cur_operator: '?',
			},
			|acc, (col, &operator_char)| {
				// Parse current operand
				let operand: u64 = (0..rows).fold(0, |acc, row| {
					let char = self.operands[row][col];
					if let Some(digit) = char.to_digit(10) { acc * 10 + digit as u64 } else { acc }
				});

				// Technically the column could actually result in 0, but it doesn't for our input so
				//  we can assume this is the column with all spaces.
				if operand == 0 {
					acc
				} else {
					if operator_char == ' ' {
						// Keep accumulating the current problem
						match acc.cur_operator {
							'+' => Acc {
								total: acc.total,
								cur: acc.cur + operand,
								cur_operator: acc.cur_operator,
							},
							'*' => Acc {
								total: acc.total,
								cur: acc.cur * operand,
								cur_operator: acc.cur_operator,
							},
							_ => acc,
						}
					} else {
						// Add the previous problem into our total, and reset for the next problem
						Acc {
							total: acc.total + acc.cur,
							cur: operand,
							cur_operator: operator_char,
						}
					}
				}
			},
		);

		Some(result.total + result.cur)
	}
}
