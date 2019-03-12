use std::iter;

use crate::cell::UNKNOWN;
use crate::cell::EMPTY;
use crate::cell::FILLED;
use crate::clues::Clues;
use crate::clues::CluesLine;
use crate::grid::Grid;
use crate::line::Line;
use crate::line::LineSize;

pub fn solve_row (
	grid: & mut Grid,
	clues: & Clues,
	row_index: LineSize,
) -> bool {

	let existing_line = & grid.rows () [row_index as usize];

	if existing_line.is_solved () {
		return false;
	}

	let combined_line = solve_line (
		& grid.rows () [row_index as usize],
		& clues.rows [row_index as usize],
	);

	let mut progress = false;

	for col_index in 0 .. combined_line.len () {

		if grid.get (row_index, col_index) != combined_line [col_index] {
			progress = true;
		}

		grid.set (row_index, col_index, combined_line [col_index]);

	}

	progress

}

pub fn solve_col (
	grid: & mut Grid,
	clues: & Clues,
	col_index: LineSize,
) -> bool {

	let existing_line = & grid.cols () [col_index as usize];

	if existing_line.is_solved () {
		return false;
	}

	let combined_line = solve_line (
		& grid.cols () [col_index as usize],
		& clues.cols [col_index as usize],
	);

	let mut progress = false;

	for row_index in 0 .. combined_line.len () {

		if grid.get (row_index, col_index) != combined_line [row_index] {
			progress = true;
		}

		grid.set (row_index, col_index, combined_line [row_index]);

	}

	progress

}

pub fn line_fits (
	existing_line: & Line,
	candidate_line: & Line,
) -> bool {

	for (existing_cell, candidate_cell) in existing_line.iter ().zip (
		candidate_line.iter (),
	) {

		if (
			* existing_cell != UNKNOWN
			&& * existing_cell != * candidate_cell
		) {
			return false;
		}

	}

	true

}

pub fn solve_line (
	existing_line: & Line,
	clues_line: & CluesLine,
) -> Line {

	let mut line_perms_iter = LinePermsIter::new (
		& clues_line,
		existing_line.len (),
	);

	loop {

		if ! line_perms_iter.next () {
			panic! ("No candidates for line");
		}

		if line_fits (& existing_line, line_perms_iter.line ()) {
			break;
		}

	}

	let mut combined_line = line_perms_iter.line ().clone ();

	while line_perms_iter.next () {

		let candidate_line = line_perms_iter.line ();

		if ! line_fits (& existing_line, line_perms_iter.line ()) {
			continue;
		}

		for (
			combined_cell,
			candidate_cell,
		) in combined_line.iter_mut ().zip (
			candidate_line.iter (),
		) {

			if (* combined_cell != * candidate_cell) {
				* combined_cell = UNKNOWN;
			}

		}

		if combined_line == * existing_line {
			break;
		}

	}

	combined_line

}

pub struct LinePermsIter <'a> {
	line: Line,
	clues: & 'a CluesLine,
	done: bool,
	offsets: Vec <LineSize>,
	spare_offset: LineSize,
}

impl <'a> LinePermsIter <'a> {

	pub fn new (
		clues: & CluesLine,
		line_size: LineSize,
	) -> LinePermsIter {

		LinePermsIter {

			line: Line::with_size (line_size),
			clues: clues,
			done: false,

			offsets: iter::repeat (0).take (
				clues.len (),
			).collect (),

			spare_offset: line_size + 1
				- clues.iter ().map (|val| { * val as LineSize }).sum::<LineSize> ()
				- clues.len () as LineSize,

		}

	}

	fn line (
		& self,
	) -> & Line {
		& self.line
	}

	fn next (
		& mut self,
	) -> bool {

		if self.done {
			return false;
		}

		// construct line

		let mut line_index = 0;

		for (
			clue,
			offset,
		) in self.clues.iter ().zip (
			self.offsets.iter (),
		) {

			if line_index > 0 {
				self.line [line_index] = EMPTY;
				line_index += 1;
			}

			for _ in 0 .. * offset {
				self.line [line_index] = EMPTY;
				line_index += 1;
			}

			for _ in 0 .. * clue {
				self.line [line_index] = FILLED;
				line_index += 1;
			}

		}

		while line_index < self.line.len () {
			self.line [line_index] = EMPTY;
			line_index += 1;
		}

		// advance state

		let mut offset = 0;

		loop {

			if offset == self.offsets.len () {
				self.done = true;
				break;
			}

			if self.spare_offset > 0 {
				self.offsets [offset] += 1;
				self.spare_offset -= 1;
				break;
			}

			self.spare_offset += self.offsets [offset];
			self.offsets [offset] = 0;
			offset += 1;

		}

		true

	}

}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn test_solve_line () {

		assert_eq! (
			solve_line (
				& Line::with_size (10),
				& vec! [ 3, 2, 3 ],
			),
			Line::from (vec! [
				FILLED, FILLED, FILLED, EMPTY, FILLED,
				FILLED, EMPTY, FILLED, FILLED, FILLED,
			]),
		);

	}

}

