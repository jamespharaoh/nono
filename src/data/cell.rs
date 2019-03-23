const ERROR: u8   = 0b00;
const EMPTY: u8   = 0b01;
const FILLED: u8  = 0b10;
const UNKNOWN: u8 = 0b11;

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
#[ repr (transparent) ]
pub struct Cell {
	bits: u8,
}

impl Cell {

	pub const ERROR: Cell = Cell { bits: ERROR };
	pub const EMPTY: Cell = Cell { bits: EMPTY };
	pub const FILLED: Cell = Cell { bits: FILLED };
	pub const UNKNOWN: Cell = Cell { bits: UNKNOWN };

	pub fn is_error (self) -> bool {
		self.bits == ERROR
	}

	pub fn is_empty (self) -> bool {
		self.bits == EMPTY
	}

	pub fn is_filled (self) -> bool {
		self.bits == FILLED
	}

	pub fn is_unknown (self) -> bool {
		self.bits == UNKNOWN
	}

	pub fn is_solved (self) -> bool {
		match self.bits {
			EMPTY => true,
			FILLED => true,
			_ => false,
		}
	}

	pub fn can_empty (self) -> bool {
		self.bits & EMPTY != ERROR
	}

	pub fn can_fill (self) -> bool {
		self.bits & FILLED != ERROR
	}

}

