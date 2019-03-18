use std::borrow;
use std::cmp;
use std::fmt;
use std::iter;
use std::ops;
use std::slice;

use crate::data::*;

#[ derive (Clone, PartialEq) ]
pub struct LineBuf {
	cells: Vec <Cell>,
}

impl LineBuf {

	pub fn with_size (
		size: LineSize,
	) -> LineBuf {

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

	pub fn is_solved (
		& self,
	) -> bool {
		self.cells.iter ().all (Cell::is_solved)
	}

	pub fn len (
		& self,
	) -> LineSize {
		self.cells.len () as LineSize
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

impl cmp::PartialEq <Line> for LineBuf {

	fn eq (& self, other: & Line) -> bool {
		self.as_ref () == other
	}

}

impl AsRef <Line> for LineBuf {

	fn as_ref (& self) -> & Line {
		& Line::new (& self.cells)
	}

}

impl AsMut <Line> for LineBuf {

	fn as_mut (& mut self) -> & mut Line {
		Line::new_mut (& mut self.cells)
	}

}

impl borrow::Borrow <Line> for LineBuf {

	fn borrow (& self) -> & Line {
		& Line::new (& self.cells)
	}

}

impl iter::FromIterator <Cell> for LineBuf {

	fn from_iter <Iter: IntoIterator <Item = Cell>> (
		iter: Iter,
	) -> LineBuf {

		LineBuf {
			cells: iter.into_iter ().collect (),
		}

	}

}

impl <'a> iter::FromIterator <& 'a Cell> for LineBuf {

	fn from_iter <Iter: IntoIterator <Item = & 'a Cell>> (
		iter: Iter,
	) -> LineBuf {

		LineBuf {
			cells: iter.into_iter ().cloned ().collect (),
		}

	}

}

impl <'a> IntoIterator for & 'a LineBuf {

	type Item = & 'a Cell;
	type IntoIter = slice::Iter <'a, Cell>;

	fn into_iter (
		self,
	) -> slice::Iter <'a, Cell> {
		self.cells.iter ()
	}

}

impl From <Vec <Cell>> for LineBuf {

	fn from (
		cells: Vec <Cell>,
	) -> LineBuf {
		LineBuf {
			cells: cells,
		}
	}

}

impl ops::Index <LineSize> for LineBuf {

	type Output = Cell;

	fn index (
		& self,
		index: LineSize,
	) -> & Cell {
		& self.cells [index as usize]
	}

}

impl ops::IndexMut <LineSize> for LineBuf {

	fn index_mut <'a> (
		& 'a mut self,
		index: LineSize,
	) -> & 'a mut Cell {
		& mut self.cells [index as usize]
	}

}

impl ops::IndexMut <ops::Range <LineSize>> for LineBuf {

	fn index_mut <'a> (
		& 'a mut self,
		range: ops::Range <LineSize>,
	) -> & 'a mut Line {
		& mut self.as_mut () [range]
	}

}

impl ops::Index <ops::Range <LineSize>> for LineBuf {

	type Output = Line;

	fn index (
		& self,
		range: ops::Range <LineSize>,
	) -> & Line {
		& self.as_ref () [range]
	}

}

impl ops::Deref for LineBuf {

	type Target = Line;

	fn deref (& self) -> & Line {
		& self [ 0 .. self.len () ]
	}

}

impl fmt::Debug for LineBuf {

	fn fmt (
		& self,
		formatter: & mut fmt::Formatter,
	) -> fmt::Result {
		fmt::Debug::fmt (self.as_ref (), formatter)
	}

}

