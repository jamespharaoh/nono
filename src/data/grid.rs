use std::iter;
use std::ops;
use std::slice;

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
	) -> slice::Iter <Cell> {

		let start = row_index as usize * self.num_cols as usize;
		let end = start + self.num_cols as usize;

		self.data [start .. end].iter ()

	}

	pub fn col <'a> (
		& 'a self,
		col_index: LineSize,
	) -> iter::StepBy <slice::Iter <Cell>> {

		let start = col_index as usize;
		let step = self.num_cols as usize;

		self.data [start ..].iter ().step_by (step)

	}

	// setters

	pub fn set_row (
		& mut self,
		row_index: LineSize,
		line: & Line,
	) {
		for (col_index, & cell) in line.iter ().enumerate () {
			self [(row_index, col_index as LineSize)] = cell;
		}
	}

	pub fn set_col (
		& mut self,
		col_index: LineSize,
		line: & Line,
	) {
		for (row_index, & cell) in line.iter ().enumerate () {
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

