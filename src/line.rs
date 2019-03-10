use std::iter;
use std::ops;
use std::slice;

use crate::cell::UNKNOWN;
use crate::cell::EMPTY;
use crate::cell::FILLED;

use crate::clues::CluesLine;

pub type LineSize = usize;

#[ derive (Clone) ]
pub struct Line {
	cells: Vec <u8>,
}

impl Line {

	pub fn with_size (
		size: LineSize,
	) -> Line {

		Line {

			cells: iter::repeat (
				UNKNOWN,
			).take (
				size as usize,
			).collect (),

		}

	}

	pub fn is_solved (
		& self,
	) -> bool {

		self.cells.iter ().all (
			|cell| * cell != UNKNOWN,
		)

	}

	pub fn len (
		& self,
	) -> LineSize {
		self.cells.len () as LineSize
	}

	pub fn iter (
		& self,
	) -> slice::Iter <u8> {
		self.cells.iter ()
	}

	pub fn iter_mut (
		& mut self,
	) -> slice::IterMut <u8> {
		self.cells.iter_mut ()
	}

}

impl ops::Index <LineSize> for Line {

	type Output = u8;

	fn index (
		& self,
		index: LineSize,
	) -> & u8 {
		& self.cells [index as usize]
	}

}

impl ops::IndexMut <LineSize> for Line {

	fn index_mut <'a> (
		& 'a mut self,
		index: LineSize,
	) -> & 'a mut u8 {
		& mut self.cells [index as usize]
	}

}

pub struct LinePermsIter <'a> {
	line: Line,
	clues: & 'a CluesLine,
	done: bool,
	offsets: Vec <LineSize>,
	spare_offset: LineSize,
}

impl <'a> LinePermsIter <'a> {

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

pub fn line_perms (
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
	line_size: LineSize,
) -> Line {

	let mut line_perms_iter = line_perms (
		& clues_line,
		line_size,
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

	}

	combined_line

}

