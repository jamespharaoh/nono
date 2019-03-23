use std::iter;
use std::ops;

use crate::data::*;

pub struct Grid {
	data: Vec <Cell>,
	num_rows: LineSize,
	num_cols: LineSize,
}

impl Grid {

	// constructors

	pub fn new (
		num_rows: LineSize,
		num_cols: LineSize,
	) -> Grid {

		Grid {
			data: iter::repeat (Cell::UNKNOWN).take (
				num_rows as usize * num_cols as usize,
			).collect (),
			num_rows: num_rows,
			num_cols: num_cols,
		}

	}

	// getters

	pub fn num_rows (& self) -> LineSize {
		self.num_rows
	}

	pub fn num_cols (& self) -> LineSize {
		self.num_cols
	}

	pub fn is_solved (& self) -> bool {
		self.data.iter ().all (
			|cell| cell.is_solved (),
		)
	}

	pub fn row <'a> (
		& 'a self,
		row_index: LineSize,
	) -> GridIter <'a> {

		let start = row_index as usize * self.num_cols as usize;
		let end = start + self.num_cols as usize;

		GridIter {
			data: & self.data [start .. end],
			step: 1,
		}

	}

	pub fn col <'a> (
		& 'a self,
		col_index: LineSize,
	) -> GridIter <'a> {

		GridIter {
			data: & self.data [col_index as usize ..],
			step: self.num_cols as usize,
		}

	}

	// setters

	pub fn set_row (
		& mut self,
		row_index: LineSize,
		line: & Line,
	) {
		for (col_index, cell) in line.iter ().enumerate () {
			self [(row_index, col_index as LineSize)] = cell;
		}
	}

	pub fn set_col (
		& mut self,
		col_index: LineSize,
		line: & Line,
	) {
		for (row_index, cell) in line.iter ().enumerate () {
			self [(row_index as LineSize, col_index)] = cell;
		}
	}

}

impl ops::Index <(LineSize, LineSize)> for Grid {

	type Output = Cell;

	fn index (
		& self,
		index: (LineSize, LineSize),
	) -> & Cell {

		let (row_index, col_index) = index;

		& self.data [
			row_index as usize * self.num_cols as usize + col_index as usize
		]

	}

}

impl ops::IndexMut <(LineSize, LineSize)> for Grid {

	fn index_mut (
		& mut self,
		index: (LineSize, LineSize),
	) -> & mut Cell {

		let (row_index, col_index) = index;

		& mut self.data [
			row_index as usize * self.num_cols as usize + col_index as usize
		]

	}

}

pub struct GridIter <'a> {
	data: & 'a [Cell],
	step: usize,
}

impl <'a> Iterator for GridIter <'a> {

	type Item = Cell;

	fn next (& mut self) -> Option <Cell> {

		let result = self.data.get (0);

		let step = usize::min (self.step, self.data.len ());
		self.data = & self.data [step .. ];

		result.cloned ()

	}

}

