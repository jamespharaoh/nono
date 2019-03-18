use std::fmt;
use std::mem;
use std::ops;
use std::slice;

use crate::data::*;

pub type LineSize = u16;

#[ derive (Eq, PartialEq) ]
#[ repr (transparent) ]
pub struct Line {
	cells: [Cell],
}

impl Line {

	pub fn new (
		cells: & [Cell],
	) -> & Line {
		unsafe {
			mem::transmute (cells)
		}
	}

	pub fn new_mut (
		cells: & mut [Cell],
	) -> & mut Line {
		unsafe {
			mem::transmute (cells)
		}
	}

	pub fn len (
		& self,
	) -> LineSize {
		self.cells.len () as LineSize
	}

	pub fn is_solved (
		& self,
	) -> bool {
		! self.cells.iter ().any (Cell::is_unknown)
	}

	pub fn iter (
		& self,
	) -> slice::Iter <Cell> {
		self.cells.iter ()
	}

	pub fn iter_mut (
		& mut self,
	) -> slice::IterMut <Cell> {
		self.cells.iter_mut ()
	}

}

impl ToOwned for Line {

	type Owned = LineBuf;

	fn to_owned (
		& self,
	) -> LineBuf {
		LineBuf::from (self.cells.to_owned ())
	}

}

impl <'a> IntoIterator for & 'a Line {

	type Item = & 'a Cell;
	type IntoIter = slice::Iter <'a, Cell>;

	fn into_iter (
		self,
	) -> slice::Iter <'a, Cell> {
		self.cells.iter ()
	}

}

impl ops::Index <LineSize> for Line {

	type Output = Cell;

	fn index (
		& self,
		index: LineSize,
	) -> & Cell {
		& self.cells [index as usize]
	}

}

impl ops::Index <ops::Range <LineSize>> for Line {

	type Output = Line;

	fn index (
		& self,
		range: ops::Range <LineSize>,
	) -> & Line {
		& Line::new (& self.cells [
			ops::Range {
				start: range.start as usize,
				end: range.end as usize,
			}
		])
	}

}

impl ops::Index <ops::RangeFrom <LineSize>> for Line {

	type Output = Line;

	fn index (
		& self,
		range: ops::RangeFrom <LineSize>,
	) -> & Line {
		& Line::new (& self.cells [
			ops::RangeFrom {
				start: range.start as usize,
			}
		])
	}

}

impl ops::IndexMut <ops::Range <LineSize>> for Line {

	fn index_mut <'a> (
		& 'a mut self,
		range: ops::Range <LineSize>,
	) -> & 'a mut Line {
		Line::new_mut (& mut self.cells [
			ops::Range {
				start: range.start as usize,
				end: range.end as usize,
			}
		])
	}

}

impl fmt::Debug for Line {

	fn fmt (
		& self,
		formatter: & mut fmt::Formatter,
	) -> fmt::Result {

		write! (
			formatter,
			"Line [{}]",
			self.cells.iter ().map (
				|cell|

				match * cell {
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
				LineBuf::from (vec! [
					Cell::UNKNOWN,
					Cell::EMPTY,
					Cell::FILLED,
					Cell::ERROR,
				]).as_ref (),
			),
			"Line [- #!]",
		);

	}

}

