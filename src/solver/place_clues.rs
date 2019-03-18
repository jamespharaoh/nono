use std::iter;

use crate::data::*;
use crate::solver::*;

pub struct PlaceCluesIter <'a> {
	cache: Cache,
	stack: Vec <Frame <'a>>,
	line: & 'a Line,
	clues: & 'a [LineSize],
	started: bool,
}

struct Cache {
	data: Vec <bool>,
	line_size: LineSize,
}

struct Frame <'a> {
	offset: LineSize,
	place_clue_iter: PlaceClueIter <'a>,
	position: LineSize,
	found: bool,
}

pub fn place_clues <'a> (
	line: & 'a Line,
	clues: & 'a [LineSize],
) -> PlaceCluesIter <'a> {

	PlaceCluesIter {
		cache: Cache::new (clues.len (), line.len ()),
		stack: Vec::with_capacity (clues.len ()),
		line: line,
		clues: clues,
		started: false,
	}

}

impl <'a> PlaceCluesIter <'a> {

	fn push (
		& mut self,
		depth: usize,
		offset: LineSize,
	) -> bool {

		if self.clues.len () == depth {
			return self.line.iter ().skip (offset as usize).all (Cell::can_empty);
		}

		if offset + self.clues [depth] > self.line.len () {
			return false;
		}

		if self.cache.is_bad (depth, offset) {
			return false;
		}

		let place_clue_iter = place_clue (
			& self.line [offset .. ],
			self.clues [depth],
		);

		self.stack.push (Frame {
			offset: offset,
			place_clue_iter: place_clue_iter,
			position: 0,
			found: false,
		});

		false

	}

}

impl <'a> Iterator for PlaceCluesIter <'a> {

	type Item = Vec <LineSize>;

	fn next (
		& mut self,
	) -> Option <Vec <LineSize>> {

		if ! self.started {

			self.started = true;

			if self.push (0, 0) {
				return Some (vec! []);
			}

		}

		loop {

			let mut frame = match self.stack.pop () {
				Some (val) => val,
				None => return None,
			};

			frame.position = match frame.place_clue_iter.next () {
				Some (val) => val + frame.offset,
				None => {

					if ! frame.found {
						self.cache.mark_bad (self.stack.len (), frame.offset);
					}

					continue;

				},
			};

			let frame_depth = self.stack.len ();
			let frame_position = frame.position;

			self.stack.push (frame);

			if self.push (
				frame_depth + 1,
				frame_position + self.clues [frame_depth] + 1,
			) {

				for frame in self.stack.iter_mut () {
					frame.found = true;
				}

				return Some (
					self.stack.iter ().map (
						|frame| frame.position
					).collect (),
				);

			}

		}

	}

}

impl Cache {

	fn new (
		num_clues: usize,
		line_size: LineSize,
	) -> Cache {

		Cache {
			data: iter::repeat (false).take (
				num_clues * line_size as usize,
			).collect (),
			line_size: line_size,
		}

	}

	fn is_bad (
		& self,
		clue_index: usize,
		cell_index: LineSize,
	) -> bool {

		self.data [
			clue_index * self.line_size as usize + cell_index as usize
		]

	}

	fn mark_bad (
		& mut self,
		clue_index: usize,
		cell_index: LineSize,
	) {

		self.data [
			clue_index * self.line_size as usize + cell_index as usize
		] = true

	}

}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn test_place_clues_1 () {

		assert_eq! (
			place_clues (
				& LineBuf::from_str (" ----").unwrap (),
				& vec! [ 3 ],
			).collect::<Vec <Vec <LineSize>>> (),
			vec! [
				vec! [ 1 ],
				vec! [ 2 ],
			],
		);

	}

	#[ test ]
	fn test_place_clues_2 () {

		assert_eq! (
			place_clues (
				& LineBuf::from_str ("----- ----").unwrap (),
				& vec! [ 3, 4 ],
			).collect::<Vec <Vec <LineSize>>> (),
			vec! [
				vec! [ 0, 6 ],
				vec! [ 1, 6 ],
				vec! [ 2, 6 ],
			],
		);

	}

	#[ test ]
	fn test_place_clues_3 () {

		assert_eq! (
			place_clues (
				& LineBuf::from_str ("--- #-----").unwrap (),
				& vec! [ 2, 3 ],
			).collect::<Vec <Vec <LineSize>>> (),
			vec! [
				vec! [ 0, 4 ],
				vec! [ 1, 4 ],
				vec! [ 4, 7 ],
			],
		);

	}

	#[ test ]
	fn test_place_clues_4 () {

		assert_eq! (
			place_clues (
				& LineBuf::from_str ("----# ----").unwrap (),
				& vec! [ 3, 4 ],
			).collect::<Vec <Vec <LineSize>>> (),
			vec! [
				vec! [ 2, 6 ],
			],
		);

	}

	#[ test ]
	fn test_place_clues_5 () {

		assert_eq! (
			place_clues (
				& LineBuf::from_str ("-#- -#-").unwrap (),
				& vec! [ 3 ],
			).collect::<Vec <Vec <LineSize>>> (),
			vec! [ ] as Vec <Vec <LineSize>>,
		);

	}

	#[ test ]
	fn test_place_clues_6 () {

		assert_eq! (
			place_clues (
				& LineBuf::from_str (" ###      #--  #--       ").unwrap (),
				& vec! [ 3, 3 ],
			).collect::<Vec <Vec <LineSize>>> (),
			vec! [ ] as Vec <Vec <LineSize>>,
		);

	}

	#[ test ]
	fn test_place_clues_7 () {

		assert_eq! (
			place_clues (
				& LineBuf::from_str (" ###      ---  #--       ").unwrap (),
				& vec! [ 3, 3 ],
			).collect::<Vec <Vec <LineSize>>> (),
			vec! [
				vec! [ 1, 15 ],
			],
		);

	}

	#[ test ]
	fn test_place_clues_8 () {

		assert_eq! (
			place_clues (
				& LineBuf::from_str ("-----").unwrap (),
				& vec! [ ],
			).collect::<Vec <Vec <LineSize>>> (),
			vec! [
				vec! [ ],
			],
		);

	}

	#[ test ]
	fn test_place_clues_9 () {

		assert_eq! (
			place_clues (
				& LineBuf::from_str ("--#--").unwrap (),
				& vec! [ ],
			).collect::<Vec <Vec <LineSize>>> (),
			vec! [ ] as Vec <Vec <LineSize>>,
		);

	}

}

