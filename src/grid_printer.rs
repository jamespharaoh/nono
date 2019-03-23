use std::io;
use std::iter;

use crate::data::*;

pub struct GridPrinter {
	top: Vec <String>,
	middle: Vec <(String, String)>,
	bottom: Vec <String>,
	shown: bool,
}

impl GridPrinter {

	pub fn new (clues: & Clues) -> GridPrinter {

		let max_row_clues = clues.rows ().map (
			|clues_line| clues_line.len (),
		).max ().unwrap_or (0);

		let max_col_clues = clues.cols ().map (
			|clues_line| clues_line.len (),
		).max ().unwrap_or (0);

		let mut top = Vec::new ();
		let mut middle = Vec::new ();
		let mut bottom = Vec::new ();

		for row in 0 .. max_col_clues {

			top.push (format! (
				"{} {}\n",
				iter::repeat ("  ").take (max_row_clues).collect::<String> (),
				(0 .. clues.num_cols ()).map (|col_index| 
					if (max_col_clues - row - 1) < clues.col (col_index).len () {
						format! ("{:2}", clues.col (col_index) [
							row - (max_col_clues - clues.col (col_index).len ())
						])
					} else {
						"  ".to_string ()
					}
				).collect::<String> (),
			));

		}

		top.push (format! (
			"{} ▄{}▄\n",
			iter::repeat ("  ").take (max_row_clues).collect::<String> (),
			iter::repeat ("▄▄").take (
				clues.num_cols () as usize,
			).collect::<String> (),
		));

		for row_index in 0 .. clues.num_rows () {

			middle.push ( (
				format! (
					"{}{} █",
					iter::repeat ("  ").take (
						max_row_clues - clues.row (row_index).len (),
					).collect::<String> (),
					clues.row (row_index).iter ().map (|clue|
						format! ("{:2}", clue),
					).collect::<String> (),
				),
				String::new (),
			) );

		}

		bottom.push (format! (
			"{} ▀{}▀\n",
			iter::repeat ("  ").take (max_row_clues).collect::<String> (),
			iter::repeat ("▀▀").take (
				clues.num_cols () as usize,
			).collect::<String> (),
		));

		GridPrinter {
			top: top,
			middle: middle,
			bottom: bottom,
			shown: false,
		}

	}

	pub fn print (
		& mut self,
		writer: & mut dyn io::Write,
		grid: & Grid,
	) -> io::Result <()> {

		for row_index in 0 .. grid.num_rows () {

			let (_, right) = & mut self.middle [row_index as usize];

			right.clear ();

			for cell in grid.row (row_index) {

				right.push_str (
					match cell {
						Cell::UNKNOWN => "▒▒",
						Cell::EMPTY => "██",
						Cell::FILLED => "  ",
						Cell::ERROR => "!!",
						_ => "??",
					},
				);

			}

			right.push_str ("█\n");

		}

		if ! self.shown {

			for line in self.top.iter () {
				write! (writer, "{}", line) ?;
			}

			for (left, right) in self.middle.iter () {
				write! (writer, "{}{}", left, right) ?;
			}

			for line in self.bottom.iter () {
				write! (writer, "{}", line) ?;
			}

			self.shown = true;

		} else {

			print! (
				"\r\x1b[{}A",
				self.top.len () + self.middle.len () + self.bottom.len (),
			);

			for _ in self.top.iter () {
				write! (writer, "\n") ?;
			}

			for (left, right) in self.middle.iter () {
				write! (writer, "{}{}", left, right) ?;
			}

			for _ in self.bottom.iter () {
				write! (writer, "\n") ?;
			}

		}

		Ok (())

	}

}

