use std::iter;

use crate::data::*;
use crate::solver::*;

pub fn solve_line (
	line: LineBuf,
	clues_line: CluesLine,
) -> Option <LineSolver> {

	LineSolver::new (
		line,
		clues_line,
	)

}

pub struct LineSolver {
	clues_line: CluesLine,
	line: LineBuf,
	sample_line: LineBuf,
	proposed_line: LineBuf,
	index: LineSize,
}

impl LineSolver {

	pub fn new (
		line: LineBuf,
		clues_line: CluesLine,
	) -> Option <LineSolver> {

		let placed_clues = match place_clues (
			& line,
			& clues_line,
		).next () {
			Some (val) => val,
			None => return None,
		};

		let sample_line = render_placed_clues (
			& clues_line,
			& placed_clues,
			line.len (),
		).collect::<LineBuf> ();

		let proposed_line = line.clone ();

		Some (LineSolver {
			clues_line: clues_line,
			line: line,
			sample_line: sample_line,
			proposed_line: proposed_line,
			index: 0,
		})

	}

	fn solve_cell (& mut self) -> Cell {

		let cell = self.line [self.index];

		if ! cell.is_unknown () {
			return cell;
		}

		let sample_cell = self.sample_line [self.index];

		self.proposed_line [self.index] = match sample_cell {
			Cell::EMPTY => Cell::FILLED,
			Cell::FILLED => Cell::EMPTY,
			Cell::UNKNOWN => return cell,
			_ => panic! (),
		};

		let placed_clues = match place_clues (
			& self.proposed_line,
			& self.clues_line,
		).next () {
			Some (val) => val,
			None => {
				self.proposed_line [self.index] = sample_cell;
				self.line [self.index] = sample_cell;
				return sample_cell;
			},
		};

		for (nested_index, placed_cell) in render_placed_clues (
			& self.clues_line,
			& placed_clues,
			self.line.len (),
		).enumerate () {

			let nested_index = nested_index as LineSize;

			if placed_cell != self.sample_line [nested_index] {
				self.sample_line [nested_index] = Cell::UNKNOWN;
			}

		}

		self.proposed_line [self.index] = Cell::UNKNOWN;

		return cell;

	}

}

impl Iterator for LineSolver {

	type Item = Cell;

	fn next (& mut self) -> Option <Cell> {

		if self.index == self.line.len () {
			return None;
		}

		let cell = self.solve_cell ();

		self.index += 1;

		Some (cell)

	}

}

#[ inline (always) ]
pub fn render_placed_clues <'a> (
	clues_line: & 'a [LineSize],
	placed_clues: & 'a [LineSize],
	line_size: LineSize,
) -> impl Iterator <Item = Cell> + 'a {

	placed_clues.iter ().cloned ().zip (
		clues_line.iter ().cloned (),
	).chain (
		vec! [(line_size, 0)],
	).scan (0, move |pos: & mut LineSize, (start, size)| {

		let result = iter::empty ().chain (
			iter::repeat (Cell::EMPTY).take ((start - * pos) as usize),
		).chain (
			iter::repeat (Cell::FILLED).take (size as usize),
		);

		* pos = start + size;

		Some (result)

	}).flatten ()

}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn test_solve_line_1 () {

		assert_eq! (
			solve_line (
				LineBuf::from_str ("----------").unwrap (),
				vec! [ 3, 2, 3 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("### ## ###").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_2 () {

		assert_eq! (
			solve_line (
				LineBuf::from_str ("----------").unwrap (),
				vec! [ 3, 4 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("--#---##--").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_3 () {

		assert_eq! (
			solve_line (
				LineBuf::from_str ("----- ----").unwrap (),
				vec! [ 3, 4 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("--#-- ####").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_4 () {

		assert_eq! (
			solve_line (
				LineBuf::from_str ("----# ----").unwrap (),
				vec! [ 3, 4 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("  ### ####").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_5 () {

		assert_eq! (
			solve_line (
				LineBuf::from_str ("-#---#----").unwrap (),
				vec! [ 3, 4 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("-##--###- ").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_6 () {

		assert_eq! (
			solve_line (
				LineBuf::from_str ("--- #-----").unwrap (),
				vec! [ 2, 3 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("--- ##----").unwrap ()),
		);

	}

}

