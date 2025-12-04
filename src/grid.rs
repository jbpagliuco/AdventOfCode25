use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord {
	pub row: isize,
	pub col: isize,
}

impl Coord {
	pub fn new(row: isize, col: isize) -> Self {
		Self {
			row,
			col,
		}
	}
}



pub struct CoordIterator {
	row_range: (isize, isize),
	col_range: (isize, isize),
	origin: Coord,
	cur: Coord,
	skip_origin: bool,
}

impl CoordIterator {
	pub fn new(origin: Coord, row_range: (isize, isize), col_range: (isize, isize), skip_origin: bool) -> Self {
		Self {
			row_range,
			col_range,
			origin,
			cur: Coord::new(row_range.0, col_range.0),
			skip_origin,
		}
	}
}

impl Iterator for CoordIterator {
	type Item = Coord;

	fn next(&mut self) -> Option<Self::Item> {
		if self.skip_origin && self.cur == Coord::new(0, 0) {
			self.cur.col += 1;
		}

		if self.cur.col > self.col_range.1 {
			self.cur.col = self.col_range.0;
			self.cur.row += 1;
		}

		if self.cur.row > self.row_range.1 {
			return None;
		}

		let row: isize = self.cur.row + self.origin.row;
		let col: isize = self.cur.col + self.origin.col;
		let coord = Coord::new(row, col);

		self.cur.col += 1;

		Some(coord)
	}
}



#[derive(Clone, Debug)]
pub struct Grid<T> {
	pub cells: Vec<Vec<T>>,
}

impl<T> Grid<T> {
	pub fn new(cells: Vec<Vec<T>>) -> Self {
		Self {
			cells,
		}
	}

	pub fn get(&self, coord: &Coord) -> Option<&T> {
		if coord.col < 0 || coord.row < 0 {
			return None;
		}

		self.cells.get(coord.col as usize).and_then(|row| row.get(coord.row as usize))
	}

	pub fn get_mut(&mut self, coord: &Coord) -> Option<&mut T> {
		if coord.col < 0 || coord.row < 0 {
			return None;
		}

		self.cells.get_mut(coord.col as usize).and_then(|row| row.get_mut(coord.row as usize))
	}

	pub fn assign(&mut self, coord: &Coord, value: T) {
		if let Some(cell) = self.get_mut(coord) {
			*cell = value;
		}
	}

	pub fn rows(&self) -> usize {
		self.cells.len()
	}

	pub fn cols(&self) -> usize {
		match self.cells.get(0) {
			Some(row) => row.len(),
			None => 0,
		}
	}

	pub fn enumerate(&self) -> GridEnumerator<'_, T> {
		GridEnumerator::new(self)
	}
}

impl<T: Display> Display for Grid<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in &self.cells {
			for cell in row {
				write!(f, "{cell}")?;
			}
			writeln!(f)?;
		}

		Ok(())
	}
}

impl<'a, T> IntoIterator for &'a Grid<T> {
	type Item = &'a T;
	type IntoIter = GridIterator<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter::new(self)
	}
}



pub struct GridEnumerator<'a, T> {
	grid: &'a Grid<T>,
	coord: Coord,
}

impl<'a, T> GridEnumerator<'a, T> {
	pub fn new(grid: &'a Grid<T>) -> Self {
		Self {
			grid,
			coord: Coord::new(0, 0),
		}
	}
}

impl<'a, T> Iterator for GridEnumerator<'a, T> {
	type Item = (Coord, &'a T);

	fn next(&mut self) -> Option<Self::Item> {
		if self.coord.col as usize >= self.grid.cols() {
			self.coord.col = 0;
			self.coord.row += 1;
		}

		let coord = self.coord;

		self.coord.col += 1;

		Some((coord, self.grid.get(&coord)?))
	}
}



pub struct GridIterator<'a, T> {
	enumerator: GridEnumerator<'a, T>,
}

impl<'a, T> GridIterator<'a, T> {
	pub fn new(grid: &'a Grid<T>) -> Self {
		Self {
			enumerator: GridEnumerator::new(grid),
		}
	}
}

impl<'a, T> Iterator for GridIterator<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		self.enumerator.next().map(|(_, value)| value)
	}
}
