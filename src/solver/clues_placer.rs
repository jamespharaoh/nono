use std::mem;

use crate::*;

pub fn place_clues <'a> (
	line: & 'a Line,
	clues: & 'a [LineSize],
) -> CluesPlacerIter <'a> {

	CluesPlacer::new (line, clues).into_iter ()

}

#[ derive (Default) ]
pub struct CluesPlacer <'a> {
	cache: Cache,
	stack: Vec <Frame <'a>>,
	line: & 'a Line,
	clues: & 'a [LineSize],
	started: bool,
}

struct Frame <'a> {
	offset: LineSize,
	clue_placer: CluePlacer <'a>,
	position: LineSize,
	found: bool,
}

impl <'a> CluesPlacer <'a> {

	pub fn new (
		line: & 'a Line,
		clues: & 'a [LineSize],
	) -> CluesPlacer <'a> {

println! ("CluesPlacer::new");

		CluesPlacer {
			cache: Cache::new (clues.len (), line.len ()),
			stack: Vec::with_capacity (clues.len ()),
			line: line,
			clues: clues,
			started: false,
		}

	}

	pub fn into_default <'b> (
		self,
	) -> CluesPlacer <'static> {

		CluesPlacer {
			cache: self.cache,
			stack: unsafe { mem::transmute (self.stack.into_default ()) },
			line: Default::default (),
			clues: Default::default (),
			started: false,
		}

	}

	pub fn into_new <'b> (
		self,
		line: & 'b Line,
		clues: & 'b [LineSize],
	) -> CluesPlacer <'b> {

		let copy = self.into_default ();

		CluesPlacer {
			cache: copy.cache.into_new (clues.len (), line.len ()),
			stack: copy.stack,
			line: line,
			clues: clues,
			started: false,
		}

	}

	pub fn advance (& mut self) -> bool {

		if ! self.started {

			self.started = true;

			if self.push (0, 0) {
				return true;
			}

		}

		loop {

			let mut frame = match self.stack.pop () {
				Some (val) => val,
				None => return false,
			};

			frame.position = match frame.clue_placer.next () {
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

				return true;

			}

		}

	}

	pub fn current (& 'a self) -> impl Iterator <Item = LineSize> + 'a {

		self.stack.iter ().map (
			|frame| frame.position
		)

	}

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

		let clue_placer = place_clue (
			& self.line [offset .. ],
			self.clues [depth],
		);

		self.stack.push (Frame {
			offset: offset,
			clue_placer: clue_placer,
			position: 0,
			found: false,
		});

		false

	}

}

impl <'a> IntoIterator for CluesPlacer <'a> {

	type Item = Vec <LineSize>;
	type IntoIter = CluesPlacerIter <'a>;

	fn into_iter (self) -> CluesPlacerIter <'a> {
		CluesPlacerIter {
			inner: self,
		}
	}

}

pub struct CluesPlacerIter <'a> {
	inner: CluesPlacer <'a>,
}

impl <'a> Iterator for CluesPlacerIter <'a> {

	type Item = Vec <LineSize>;

	fn next (& mut self) -> Option <Vec <LineSize>> {
		if self.inner.advance () {
			Some (self.inner.current ().collect ())
		} else {
			None
		}

	}

}

#[ derive (Default) ]
struct Cache {
	data: Vec <bool>,
	line_size: LineSize,
}

impl Cache {

	fn new (
		num_clues: usize,
		line_size: LineSize,
	) -> Cache {

		let cache: Cache = Default::default ();

		cache.into_new (
			num_clues,
			line_size,
		)

	}

	fn into_new (
		mut self,
		num_clues: usize,
		line_size: LineSize,
	) -> Cache {

		self.data.truncate (0);
		self.data.resize (num_clues * line_size as usize, false);

		self.line_size = line_size;

		self

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

