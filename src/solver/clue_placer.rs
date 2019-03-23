use crate::data::*;

pub fn place_clue (
	line: & Line,
	size: LineSize,
) -> CluePlacer {

	CluePlacer::new (
		line,
		size,
	)

}

pub struct CluePlacer <'a> {
	line: & 'a Line,
	size: LineSize,
	start: LineSize,
}

impl <'a> CluePlacer <'a> {

	fn new (
		line: & Line,
		size: LineSize,
	) -> CluePlacer {

		CluePlacer {
			line: line,
			size: size,
			start: 0,
		}

	}

}

impl <'a> Iterator for CluePlacer <'a> {

	type Item = LineSize;

	fn next (
		& mut self,
	) -> Option <LineSize> {

		loop {

			if self.start + self.size > self.line.len () {
				return None;
			}

			if self.start != 0 && ! self.line [self.start - 1].can_empty () {
				return None;
			}

			let result = if self.line.iter ().skip (
				self.start as usize,
			).take (
				self.size as usize,
			).all (Cell::can_fill) && (false
				|| self.line.len () == self.start + self.size
				|| self.line [self.start + self.size].can_empty ()
			) {
				Some (self.start)
			} else {
				None
			};

			self.start += 1;

			if result.is_some () {
				return result;
			}

		}

	}

}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn test_place_clue_1 () {

		assert_eq! (
			place_clue (
				& LineBuf::from_str (" -#---").unwrap (),
				3,
			).collect::<Vec <LineSize>> (),
			vec! [ 1, 2 ],
		);

	}

	#[ test ]
	fn test_place_clue_2 () {

		assert_eq! (
			place_clue (
				& LineBuf::from_str ("--- -#----").unwrap (),
				2,
			).collect::<Vec <LineSize>> (),
			vec! [ 0, 1, 4, 5 ],
		);

	}

}

