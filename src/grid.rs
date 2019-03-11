use std::iter;

use crate::cell::UNKNOWN;
use crate::cell::ERROR;
use crate::cell::EMPTY;
use crate::cell::FILLED;

use crate::line::Line;
use crate::line::LineSize;

pub struct Grid {
	rows: Vec <Line>,
	cols: Vec <Line>,
}

impl Grid {

	// constructors

	pub fn new (
		num_rows: LineSize,
		num_cols: LineSize,
	) -> Grid {

		Grid {

			rows: iter::repeat (
				Line::with_size (num_cols),
			).take (num_rows as usize).collect (),

			cols: iter::repeat (
				Line::with_size (num_rows),
			).take (num_cols as usize).collect (),

		}

	}

	// getters

	pub fn num_rows (
		& self,
	) -> LineSize {
		self.rows.len () as LineSize
	}

	pub fn num_cols (
		& self,
	) -> LineSize {
		self.cols.len () as LineSize
	}

	pub fn rows (
		& self,
	) -> & [Line] {
		& self.rows
	}

	pub fn cols (
		& self,
	) -> & [Line] {
		& self.cols
	}

	pub fn is_solved (
		& self,
	) -> bool {
		self.rows.iter ().all (
			|row| row.is_solved (),
		)
	}

	// setters

	pub fn get (
		& self,
		row_index: LineSize,
		col_index: LineSize,
	) -> u8 {
		self.rows [row_index] [col_index]
	}

	pub fn set (
		& mut self,
		row_index: LineSize,
		col_index: LineSize,
		value: u8,
	) {
		self.rows [row_index] [col_index] = value;
		self.cols [col_index] [row_index] = value;
	}

	// misc

	pub fn print (
		& self,
	) {

		println! (
			"┌{}┐",
			iter::repeat ("─").take (
				self.cols.len () * 2,
			).collect::<String> ());

		for row in & self.rows {

			println! (
				"│{}│",
				row.iter ().map (
					|cell|

					match * cell {
						UNKNOWN => "░░",
						EMPTY => "  ",
						FILLED => "██",
						ERROR => "!!",
						_ => "??",
					}

				).collect::<String> ());

		}

		println! (
			"└{}┘",
			iter::repeat ("─").take (
				self.cols.len () * 2,
			).collect::<String> ());

	}

}

