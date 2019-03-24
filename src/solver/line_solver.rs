use std::iter;
use std::mem;

use crate::*;

pub fn solve_line <
	LineIter: IntoIterator <Item = Cell>,
> (
	line_iter: LineIter,
	clues_line: & CluesLine,
) -> Option <LineSolverIter> {

	LineSolver::new (
		line_iter,
		clues_line,
	).ok ().map (LineSolver::into_iter)

}

#[ derive (Default) ]
pub struct LineSolver {
	clues_line: CluesLine,
	line: LineBuf,
	sample_line: LineBuf,
	proposed_line: LineBuf,
	index: LineSize,
	clues_placer: CluesPlacer <'static>,
}

impl LineSolver {

	pub fn new <
		LineIter: IntoIterator <Item = Cell>,
	> (
		line_iter: LineIter,
		clues_line: & CluesLine,
	) -> Result <LineSolver, LineSolver> {

		let line_solver: LineSolver = Default::default ();

		line_solver.into_new (line_iter, clues_line)

	}

	pub fn into_default (
		self,
	) -> LineSolver {

		LineSolver {
			clues_line: self.clues_line.into_default (),
			line: self.line.into_default (),
			sample_line: self.sample_line.into_default (),
			proposed_line: self.proposed_line.into_default (),
			index: 0,
			clues_placer: self.clues_placer.into_default (),
		}

	}

	pub fn into_new <
		LineIter: IntoIterator <Item = Cell>,
	> (
		self,
		line_iter: LineIter,
		clues_line: & CluesLine,
	) -> Result <LineSolver, LineSolver> {

		let line = self.line.into_copy_of (line_iter);

		let (clues_placer, sample_line) = {

			let mut clues_placer = self.clues_placer.into_new (
				& line,
				& clues_line,
			);

			if ! clues_placer.advance () {
				return Err (Default::default ()); // TODO
			};

			let sample_line = self.sample_line.into_copy_of (
				render_placed_clues (
					clues_line.iter ().cloned (),
					clues_placer.current (),
					line.len (),
				),
			);

			(clues_placer.into_default (), sample_line)

		};

		let proposed_line = self.proposed_line.into_copy_of (& line);

		Ok (LineSolver {
			clues_line: self.clues_line.into_default ().into_extend (
				clues_line.iter ().cloned (),
			),
			line: line,
			sample_line: sample_line,
			proposed_line: proposed_line,
			index: 0,
			clues_placer: clues_placer,
		})

	}

	pub fn next (& mut self) -> Option <Cell> {

		if self.index == self.line.len () {
			return None;
		}

		let cell = self.solve_cell ();

		self.index += 1;

		Some (cell)

	}

	fn solve_cell (& mut self) -> Cell {

		// return existing cell if known

		let existing_cell = self.line [self.index];

		if ! existing_cell.is_unknown () {
			return existing_cell;
		}

		// propose inverting the sample cell

		let sample_cell = self.sample_line [self.index];

		self.proposed_line [self.index] = match sample_cell {
			Cell::EMPTY => Cell::FILLED,
			Cell::FILLED => Cell::EMPTY,
			Cell::UNKNOWN => return existing_cell,
			_ => panic! (),
		};

		// try placing clues

		let mut clues_placer = Default::default ();
		mem::swap (& mut clues_placer, & mut self.clues_placer);

		let mut clues_placer = clues_placer.into_new (
			& self.proposed_line,
			& self.clues_line,
		);

		// if it fails the sample cell must be correct

		if ! clues_placer.advance () {

			self.clues_placer = clues_placer.into_default ();

			self.proposed_line [self.index] = sample_cell;
			self.line [self.index] = sample_cell;

			return sample_cell;

		}

		// remove proposed cells which contradict placed clues

		for (nested_index, placed_cell) in render_placed_clues (
			self.clues_line.iter ().cloned (),
			clues_placer.current (),
			self.line.len (),
		).enumerate () {

			let nested_index = nested_index as LineSize;

			if placed_cell != self.sample_line [nested_index] {
				self.sample_line [nested_index] = Cell::UNKNOWN;
			}

		}

		// reset the proposed cell

		self.clues_placer = clues_placer.into_default ();

		self.proposed_line [self.index] = Cell::UNKNOWN;

		return existing_cell;

	}

}

impl IntoIterator for LineSolver {

	type Item = Cell;
	type IntoIter = LineSolverIter;

	fn into_iter (self) -> LineSolverIter {
		LineSolverIter {
			inner: self,
		}
	}

}

pub struct LineSolverIter {
	inner: LineSolver,
}

impl Iterator for LineSolverIter {

	type Item = Cell;

	fn next (& mut self) -> Option <Cell> {
		self.inner.next ()
	}

}

#[ inline (always) ]
pub fn render_placed_clues <
	'a,
	CluesLine: IntoIterator <Item = LineSize>,
	PlacedClues: IntoIterator <Item = LineSize>,
> (
	clues_line: CluesLine,
	placed_clues: PlacedClues,
	line_size: LineSize,
) -> impl Iterator <Item = Cell> {

	placed_clues.into_iter ().zip (
		clues_line.into_iter (),
	).chain (
		iter::once ((line_size, 0)),
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
				& LineBuf::from_str ("----------").unwrap (),
				& vec! [ 3, 2, 3 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("### ## ###").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_2 () {

		assert_eq! (
			solve_line (
				& LineBuf::from_str ("----------").unwrap (),
				& vec! [ 3, 4 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("--#---##--").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_3 () {

		assert_eq! (
			solve_line (
				& LineBuf::from_str ("----- ----").unwrap (),
				& vec! [ 3, 4 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("--#-- ####").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_4 () {

		assert_eq! (
			solve_line (
				& LineBuf::from_str ("----# ----").unwrap (),
				& vec! [ 3, 4 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("  ### ####").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_5 () {

		assert_eq! (
			solve_line (
				& LineBuf::from_str ("-#---#----").unwrap (),
				& vec! [ 3, 4 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("-##--###- ").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_6 () {

		assert_eq! (
			solve_line (
				& LineBuf::from_str ("--- #-----").unwrap (),
				& vec! [ 2, 3 ],
			).map (Iterator::collect),
			Some (LineBuf::from_str ("--- ##----").unwrap ()),
		);

	}

}

