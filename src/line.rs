use std::fmt;
use std::iter;
use std::ops;
use std::slice;
use std::vec;

use crate::cell::EMPTY;
use crate::cell::ERROR;
use crate::cell::FILLED;
use crate::cell::UNKNOWN;

pub type LineSize = u16;

#[ derive (Clone, PartialEq) ]
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

impl ops::Index <ops::Range <LineSize>> for Line {

	type Output = [u8];

	fn index (
		& self,
		range: ops::Range <LineSize>,
	) -> & [u8] {
		& self.cells [ops::Range {
			start: range.start as usize,
			end: range.end as usize,
		}]
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

