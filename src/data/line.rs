use std::fmt;
use std::iter::Cloned;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use std::ops::RangeFrom;
use std::ops::RangeTo;
use std::slice;

use crate::data::*;

pub type LineSize = u16;

#[ derive (Eq, PartialEq) ]
#[ repr (transparent) ]
pub struct Line {
	cells: [Cell],
}

impl Line {

	pub fn new (cells: & [Cell]) -> & Line {
		unsafe { mem::transmute (cells) }
	}

	pub fn new_mut (cells: & mut [Cell]) -> & mut Line {
		unsafe { mem::transmute (cells) }
	}

	pub fn cells (& self) -> & [Cell] {
		& self.cells
	}

	pub fn len (& self) -> LineSize {
		self.cells.len () as LineSize
	}

	pub fn is_solved (& self) -> bool {
		! self.iter ().any (Cell::is_unknown)
	}

	pub fn iter (& self) -> Cloned <slice::Iter <'_, Cell>> {
		self.cells.iter ().cloned ()
	}

	pub fn iter_mut (& mut self) -> slice::IterMut <'_, Cell> {
		self.cells.iter_mut ()
	}

}

impl <'a> Default for & 'a Line {

	fn default () -> & 'a Line {
		Line::new (Default::default ())
	}

}

impl ToOwned for Line {

	type Owned = LineBuf;

	fn to_owned (& self) -> LineBuf {
		LineBuf::from (self.cells.to_owned ())
	}

}

impl Index <LineSize> for Line {

	type Output = Cell;

	fn index (& self, index: LineSize) -> & Cell {
		& self.cells [index as usize]
	}

}

impl Index <Range <LineSize>> for Line {

	type Output = Line;

	fn index (& self, range: Range <LineSize>) -> & Line {
		& Line::new (& self.cells [
			Range {
				start: range.start as usize,
				end: range.end as usize,
			}
		])
	}

}

impl Index <RangeFrom <LineSize>> for Line {

	type Output = Line;

	fn index (& self, range: RangeFrom <LineSize>) -> & Line {
		& Line::new (& self.cells [
			RangeFrom {
				start: range.start as usize,
			}
		])
	}

}

impl Index <RangeTo <LineSize>> for Line {

	type Output = Line;

	fn index (& self, range: RangeTo <LineSize>) -> & Line {
		& Line::new (& self.cells [
			RangeTo {
				end: range.end as usize,
			}
		])
	}

}

impl IndexMut <LineSize> for Line {

	fn index_mut <'a> (& 'a mut self, index: LineSize) -> & 'a mut Cell {
		& mut self.cells [index as usize]
	}

}

impl IndexMut <Range <LineSize>> for Line {

	fn index_mut <'a> (& 'a mut self, range: Range <LineSize>) -> & 'a mut Line {
		Line::new_mut (& mut self.cells [
			Range {
				start: range.start as usize,
				end: range.end as usize,
			}
		])
	}

}

impl fmt::Debug for Line {

	fn fmt (& self, formatter: & mut fmt::Formatter <'_>) -> fmt::Result {

		write! (
			formatter,
			"[{}]",
			self.cells.iter ().map (
				|& cell|

				match cell {
					Cell::UNKNOWN => "-",
					Cell::EMPTY => " ",
					Cell::FILLED => "#",
					Cell::ERROR => "!",
					_ => "?",
				}

			).collect::<String> (),
		) ?;

		Ok (())

	}

}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn test_line_debug () {

		assert_eq! (
			format! (
				"{:?}",
				Line::new (& vec! [
					Cell::UNKNOWN,
					Cell::EMPTY,
					Cell::FILLED,
					Cell::ERROR,
				]),
			),
			"[- #!]",
		);

	}

}

