use std::borrow::Borrow;
use std::fmt;
use std::iter;
use std::iter::FromIterator;
use std::ops::Deref;
use std::ops::DerefMut;
use std::slice;

use crate::*;

#[ derive (Default, PartialEq) ]
pub struct LineBuf {
	cells: Vec <Cell>,
}

impl LineBuf {

	pub fn into_copy_of <
		MyIntoLine: IntoLine,
	> (
		self,
		into_line: MyIntoLine,
	) -> LineBuf {

		LineBuf {
			cells: self.cells.into_default ().into_extend (into_line.into ()),
		}

	}

	pub fn with_size (size: LineSize) -> LineBuf {

		LineBuf {
			cells: iter::repeat (Cell::UNKNOWN).take (size as usize).collect (),
		}

	}

	pub fn from_str (
		source: & str,
	) -> Option <LineBuf> {

		Some (LineBuf {

			cells: match source.bytes ().map (
				|ch| match ch {
					b'-' => Some (Cell::UNKNOWN),
					b' ' => Some (Cell::EMPTY),
					b'#' => Some (Cell::FILLED),
					b'!' => Some (Cell::EMPTY),
					_ => None,
				}
			).collect () {
				Some (val) => val,
				None => return None,
			},

		})

	}

	pub fn capacity (& self) -> LineSize {
		self.cells.capacity () as LineSize
	}

}

impl Borrow <Line> for LineBuf {

	fn borrow (& self) -> & Line {
		self.deref ()
	}

}

impl Deref for LineBuf {

	type Target = Line;

	fn deref (& self) -> & Line {
		Line::new (& self.cells)
	}

}

impl DerefMut for LineBuf {

	fn deref_mut (& mut self) -> & mut Line {
		Line::new_mut (& mut self.cells)
	}

}

impl FromIterator <Cell> for LineBuf {

	fn from_iter <Iter: IntoIterator <Item = Cell>> (
		iter: Iter,
	) -> LineBuf {

		LineBuf {
			cells: iter.into_iter ().collect (),
		}

	}

}

impl IntoDefault for LineBuf {

	fn into_default (self) -> LineBuf {
		LineBuf {
			cells: self.cells.into_default (),
		}
	}

}

impl <'a> IntoIterator for & 'a LineBuf {

	type Item = & 'a Cell;
	type IntoIter = slice::Iter <'a, Cell>;

	fn into_iter (self) -> slice::Iter <'a, Cell> {
		self.cells.iter ()
	}

}

impl From <Vec <Cell>> for LineBuf {

	fn from (cells: Vec <Cell>) -> LineBuf {
		LineBuf {
			cells: cells,
		}
	}

}

impl fmt::Debug for LineBuf {

	fn fmt (
		& self,
		formatter: & mut fmt::Formatter,
	) -> fmt::Result {
		fmt::Debug::fmt (& self, formatter)
	}

}

