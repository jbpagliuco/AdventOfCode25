use std::{
	fmt::Debug,
	time::Instant,
};

mod day1;
mod day2;

use day2 as today; // UPDATE ME EACH DAY!

fn main() {
	run_solver::<today::Part1>(today::FILENAME, 1);
	run_solver::<today::Part2>(today::FILENAME, 2);
}



pub trait Solver {
	type Output: Debug + PartialEq;

	fn parse(input: String) -> Self;
	fn expected_output() -> Option<Self::Output>;

	fn solve(&self) -> Option<Self::Output>;
}

fn run_solver<T: Solver>(filename: &str, part: i32) {
	println!("Running part {part}...");

	// Parse input
	let filename = filename.replace("src\\", "input\\").replace(".rs", ".txt");
	let input = read_input(&filename);

	// Solve
	let timer = Instant::now();
	let solver = T::parse(input);
	let answer = solver.solve();
	let elapsed = timer.elapsed();

	// Print output
	println!("Expected Answer: {:?}", T::expected_output());
	println!("Actual Answer:   {answer:?}");
	println!("{}", if answer == T::expected_output() { "Correct!" } else { "Wrong!" });
	println!("Took {elapsed:?}");
	println!();
}

/// Read input file line into a String.
pub fn read_input(filename: &str) -> String {
	let cwd = std::env::current_dir().unwrap();
	let path = cwd.join(filename);
	std::fs::read_to_string(path).unwrap_or_default()
}
