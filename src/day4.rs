use crate::{
	Solver,
	grid::{
		Coord,
		CoordIterator,
		Grid,
	},
};

#[allow(dead_code)]
pub const FILENAME: &str = file!();

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
	Empty,
	Paper,
}

// Part 1
pub struct Part1 {
	grid: Grid<Cell>,
}

impl Solver for Part1 {
	type Output = i32;

	fn parse(input: String) -> Self {
		let cells = input
			.lines()
			.map(|line| line.chars().map(|c| if c == '@' { Cell::Paper } else { Cell::Empty }).collect())
			.collect();

		Self {
			grid: Grid::new(cells),
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(1505)
	}

	fn solve(&self) -> Option<Self::Output> {
		let mut count = 0;

		for (coord, cell) in self.grid.enumerate() {
			if cell != &Cell::Paper {
				continue;
			}

			let neighbors = CoordIterator::new(coord, (-1, 1), (-1, 1), true)
				.filter(|neighbor| self.grid.get(neighbor) == Some(&Cell::Paper))
				.count();

			if neighbors < 4 {
				count += 1;
			}
		}

		Some(count)
	}
}



// Part 2
pub struct Part2 {
	grid: Grid<Cell>,
}

impl Solver for Part2 {
	type Output = usize;

	fn parse(input: String) -> Self {
		let cells = input
			.lines()
			.map(|line| line.chars().map(|c| if c == '@' { Cell::Paper } else { Cell::Empty }).collect())
			.collect();

		Self {
			grid: Grid::new(cells),
		}
	}

	fn expected_output() -> Option<Self::Output> {
		Some(9182)
	}

	fn solve(&self) -> Option<Self::Output> {
		let mut count = 0;

		let mut grid = self.grid.clone();

		loop {
			let mut coords_to_empty: Vec<Coord> = vec![];

			for (coord, cell) in grid.enumerate() {
				if cell == &Cell::Empty {
					continue;
				}

				let neighbors = CoordIterator::new(coord, (-1, 1), (-1, 1), true)
					.filter(|neighbor| grid.get(neighbor) == Some(&Cell::Paper))
					.count();

				if neighbors < 4 {
					coords_to_empty.push(coord);
				}
			}

			if coords_to_empty.len() == 0 {
				break;
			}

			count += coords_to_empty.len();
			for coord in coords_to_empty {
				grid.assign(&coord, Cell::Empty);
			}
		}

		Some(count)
	}
}
