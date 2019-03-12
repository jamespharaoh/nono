use std::iter;
use std::ops;
use std::slice;

use crate::cell::UNKNOWN;

pub type LineSize = u16;

#[ derive (Clone, Debug, PartialEq) ]
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

	pub fn is_unknown (
		& self,
	) -> bool {

		self.cells.iter ().all (
			|cell| * cell == UNKNOWN,
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

impl From <Vec <u8>> for Line {

	fn from (
		cells: Vec <u8>,
	) -> Line {
		Line {
			cells: cells,
		}
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

