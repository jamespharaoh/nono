use std::iter;

use crate::cell::UNKNOWN;
use crate::cell::ERROR;
use crate::cell::EMPTY;
use crate::cell::FILLED;

use crate::line::Line;
use crate::line::LineSize;

pub struct Grid {
	pub rows: Vec <Line>,
	pub cols: Vec <Line>,
}

impl Grid {

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

	pub fn is_solved (
		& self,
	) -> bool {
		self.rows.iter ().all (
			|row| row.is_solved (),
		)
	}

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

