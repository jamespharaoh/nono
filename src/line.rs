use std::fmt;
use std::iter;
use std::ops;
use std::slice;

use crate::cell::EMPTY;
use crate::cell::ERROR;
use crate::cell::FILLED;
use crate::cell::UNKNOWN;

pub type LineSize = u16;

#[ derive (Clone, PartialEq) ]
pub struct Line {
	cells: Vec <u8>,
}

pub struct LineRef {
	cells: [u8],
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

	pub fn from_str (
		source: & str,
	) -> Option <Line> {

		Some (Line {

			cells: match source.bytes ().map (
				|ch| match ch {
					b'-' => Some (UNKNOWN),
					b' ' => Some (EMPTY),
					b'#' => Some (FILLED),
					b'!' => Some (ERROR),
					_ => None,
				}
			).collect () {
				Some (val) => val,
				None => return None,
			},

		})

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

impl LineRef {

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

impl ToOwned for LineRef {

	type Owned = Line;

	fn to_owned (
		& self,
	) -> Line {

		Line {
			cells: self.cells.to_owned (),
		}

	}

}

impl ::std::borrow::Borrow <LineRef> for Line {

	fn borrow (
		& self,
	) -> & LineRef {
		& self [ 0 .. self.len () ]
	}

}

impl iter::FromIterator <u8> for Line {

	fn from_iter <Iter: IntoIterator <Item = u8>> (
		iter: Iter,
	) -> Line {

		Line {
			cells: iter.into_iter ().collect (),
		}

	}

}

impl <'a> iter::FromIterator <& 'a u8> for Line {

	fn from_iter <Iter: IntoIterator <Item = &'a u8>> (
		iter: Iter,
	) -> Line {

		Line {
			cells: iter.into_iter ().cloned ().collect (),
		}

	}

}

impl <'a> IntoIterator for & 'a Line {

	type Item = & 'a u8;
	type IntoIter = slice::Iter <'a, u8>;

	fn into_iter (
		self,
	) -> slice::Iter <'a, u8> {
		self.cells.iter ()
	}

}

impl <'a> IntoIterator for & 'a LineRef {

	type Item = & 'a u8;
	type IntoIter = slice::Iter <'a, u8>;

	fn into_iter (
		self,
	) -> slice::Iter <'a, u8> {
		self.cells.iter ()
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
					UNKNOWN => "-",
					EMPTY => " ",
					FILLED => "#",
					ERROR => "!",
					_ => "?",
				}

			).collect::<String> (),
		) ?;

		Ok (())

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

impl ops::Index <LineSize> for LineRef {

	type Output = u8;

	fn index (
		& self,
		index: LineSize,
	) -> & u8 {
		& self.cells [index as usize]
	}

}

impl ops::Index <ops::Range <LineSize>> for Line {

	type Output = LineRef;

	fn index (
		& self,
		range: ops::Range <LineSize>,
	) -> & LineRef {

		unsafe {

			::std::mem::transmute (
				& self.cells [ops::Range {
					start: range.start as usize,
					end: range.end as usize,
				}],
			)

		}

	}

}

impl ops::Index <ops::Range <LineSize>> for LineRef {

	type Output = LineRef;

	fn index (
		& self,
		range: ops::Range <LineSize>,
	) -> & LineRef {

		unsafe {

			::std::mem::transmute (
				& self.cells [ops::Range {
					start: range.start as usize,
					end: range.end as usize,
				}],
			)

		}

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

impl ops::IndexMut <ops::Range <LineSize>> for Line {

	fn index_mut <'a> (
		& 'a mut self,
		range: ops::Range <LineSize>,
	) -> & 'a mut LineRef {

		unsafe {

			::std::mem::transmute (
				& mut self.cells [ops::Range {
					start: range.start as usize,
					end: range.end as usize,
				}],
			)

		}

	}

}

impl ops::Deref for Line {

	type Target = LineRef;

	fn deref (
		& self,
	) -> & LineRef {
		& self [ 0 .. self.len () ]
	}

}

impl <'a> From <& 'a Line> for & 'a LineRef {

	fn from (
		line: & Line,
	) -> & LineRef {

		unsafe {

			::std::mem::transmute (
				line.cells.as_slice (),
			)

		}

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
				Line::from (vec! [ UNKNOWN, EMPTY, FILLED, ERROR ]),
			),
			"Line [- #!]",
		);

	}

}

