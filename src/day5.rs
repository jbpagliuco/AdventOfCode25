use crate::Solver;

#[allow(dead_code)]
pub const FILENAME: &str = file!();



#[derive(Copy, Clone)]
pub struct Range<T> {
	min: T,
	last: T,
}

impl<T> Range<T> {
	pub fn new(min: T, last: T) -> Self {
		Self {
			min,
			last,
		}
	}

	pub fn contains(&self, value: &T) -> bool
	where
		T: PartialOrd,
	{
		value >= &self.min && value <= &self.last
	}
}



// Part 1
pub struct Part1 {
	ranges: Vec<Range<u64>>,
	ingredients: Vec<u64>,
}

impl Solver for Part1 {
	type Output = usize;

	fn parse(input: String) -> Self {
		let mut ranges = vec![];
		let mut ingredients = vec![];

		let mut is_parsing_fresh_ingredient_ranges = true;
		for line in input.lines() {
			if line.is_empty() {
				is_parsing_fresh_ingredient_ranges = false;
				continue;
			}

			if is_parsing_fresh_ingredient_ranges {
				let (min, last) = line.split_once("-").unwrap();
				let min = min.parse().unwrap();
				let last = last.parse().unwrap();
				ranges.push(Range::new(min, last));
			} else {
				ingredients.push(line.parse().unwrap());
			}
		}

		Self {
			ranges,
			ingredients,
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(707)
	}

	fn solve(&self) -> Option<Self::Output> {
		Some(
			self.ingredients
				.iter()
				.filter(|ingredient| self.ranges.iter().any(|range| range.contains(*ingredient)))
				.count(),
		)
	}
}



// Part 2
pub struct Part2 {
	ranges: Vec<Range<u64>>,
}

impl Solver for Part2 {
	type Output = u64;

	fn parse(input: String) -> Self {
		let mut ranges = vec![];

		for line in input.lines() {
			if line.is_empty() {
				break;
			}

			let (min, last) = line.split_once("-").unwrap();
			let min = min.parse().unwrap();
			let last = last.parse().unwrap();
			ranges.push(Range::new(min, last));
		}

		Self {
			ranges,
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(361615643045059)
	}

	fn solve(&self) -> Option<Self::Output> {
		let mut ranges = self.ranges.clone();
		ranges.sort_by_key(|range| range.min);

		let mut merged = vec![ranges[0]];

		for range in ranges.iter().skip(1) {
			let last = merged.last_mut().unwrap();

			if range.min <= last.last {
				last.last = last.last.max(range.last);
			} else {
				merged.push(range.clone());
			}
		}

		Some(merged.iter().map(|range| range.last - range.min + 1).sum())
	}
}
