use std::iter;

use crate::cell::UNKNOWN;
use crate::cell::EMPTY;
use crate::cell::FILLED;
use crate::clues::Clues;
use crate::clues::CluesLine;
use crate::grid::Grid;
use crate::line::Line;
use crate::line::LineRef;
use crate::line::LineSize;

#[ inline (always) ]
pub fn solve_row (
	grid: & mut Grid,
	clues: & Clues,
	row_index: LineSize,
) -> bool {

	let existing_line = & grid.rows () [row_index as usize];

	if existing_line.is_solved () {
		return false;
	}

	let combined_line = solve_line (
		& grid.rows () [row_index as usize],
		& clues.rows [row_index as usize],
	).unwrap ();

	let mut progress = false;

	for col_index in 0 .. combined_line.len () {

		if grid.get (row_index, col_index) != combined_line [col_index] {
			progress = true;
		}

		grid.set (row_index, col_index, combined_line [col_index]);

	}

	progress

}

#[ inline (always) ]
pub fn solve_col (
	grid: & mut Grid,
	clues: & Clues,
	col_index: LineSize,
) -> bool {

	let existing_line = & grid.cols () [col_index as usize];

	if existing_line.is_solved () {
		return false;
	}

	let combined_line = solve_line (
		& grid.cols () [col_index as usize],
		& clues.cols [col_index as usize],
	).unwrap ();

	let mut progress = false;

	for row_index in 0 .. combined_line.len () {

		if grid.get (row_index, col_index) != combined_line [row_index] {
			progress = true;
		}

		grid.set (row_index, col_index, combined_line [row_index]);

	}

	progress

}

fn place_clues_real (
	result: & mut Vec <(LineSize, LineSize)>,
	line: & LineRef,
	clues_line: & [LineSize],
	offset: LineSize,
	cache: & mut Vec <Vec <Option <bool>>>,
) -> bool {

	let result_len = result.len ();

	if clues_line.is_empty () {

		return line.iter ().skip (offset as usize).all (
			|cell| * cell == UNKNOWN || * cell == EMPTY
		);

	}

	let my_size = clues_line [0];

	if offset + my_size > line.len () {
		return false;
	}

	if let Some (cached) = cache [clues_line.len () as usize - 1] [offset as usize] {
		return cached;
	}

	for my_start in offset ..= line.len () - my_size {

		if line.iter ().skip (
			my_start as usize,
		).take (
			my_size as usize,
		).all (
			|cell| * cell == UNKNOWN || * cell == FILLED
		) && (false
			|| line.len () == my_start + my_size
			|| line [my_start + my_size] == UNKNOWN
			|| line [my_start + my_size] == EMPTY
		) {

			// handle clue at end of line

			if my_start + my_size == line.len () {
				if clues_line.len () == 1 {
					result.push ((my_start, my_start + my_size));
					cache [clues_line.len () as usize - 1] [offset as usize] = Some (true);
					return true;
				} else {
					cache [clues_line.len () as usize - 1] [offset as usize] = Some (false);
					return false;
				};
			}

			// recurse

			result.push ((my_start, my_start + my_size));

			if place_clues_real (
				result,
				line,
				& clues_line [1 ..],
				my_start + my_size + 1,
				cache,
			) {
				cache [clues_line.len () as usize - 1] [offset as usize] = Some (true);
				return true;
			} else {
				result.truncate (result_len);
			}

		}

		// handle can't fill cell

		if ! (false
			|| line [my_start] == UNKNOWN
			|| line [my_start] == EMPTY
		) {
			cache [clues_line.len () as usize - 1] [offset as usize] = Some (false);
			return false;
		}

	}

	false

}

#[ inline (always) ]
fn place_clues (
	result: & mut Vec <(LineSize, LineSize)>,
	existing_line: & LineRef,
	clues_line: & [LineSize],
) -> bool {

	let mut cache = iter::repeat (
		iter::repeat (None).take (existing_line.len () as usize).collect (),
	).take (clues_line.len ()).collect::<Vec <Vec <Option <bool>>>> ();

	place_clues_real (
		result,
		existing_line,
		clues_line,
		0,
		& mut cache,
	)

}

#[ inline (always) ]
pub fn render_placed_clues <'a> (
	placed_clues: & 'a [(LineSize, LineSize)],
	line_size: LineSize,
) -> impl Iterator <Item = u8> + 'a {

	placed_clues.iter ().cloned ().chain (
		vec! [(line_size, line_size)],
	).scan (0, move |pos: & mut LineSize, (start, end)| {

		let result = iter::empty ().chain (
			iter::repeat (EMPTY).take ((start - * pos) as usize),
		).chain (
			iter::repeat (FILLED).take ((end - start) as usize),
		);

		* pos = end;

		Some (result)

	}).flatten ()

}

pub fn solve_line (
	existing_line: & LineRef,
	clues_line: & CluesLine,
) -> Option <Line> {

	let mut placed_clues = Vec::with_capacity (clues_line.len ());

	// generate sample line

	if ! place_clues (
		& mut placed_clues,
		existing_line,
		clues_line,
	) {
		return None;
	}

	let mut sample_line = render_placed_clues (
		& placed_clues,
		existing_line.len (),
	).collect::<Line> ();

	for & (start, end) in placed_clues.iter () {

		for cell in sample_line [ start .. end ].iter_mut () {
			* cell = FILLED;
		}

	}

	let mut proposed_line = existing_line.to_owned ();
	let mut solved_line = existing_line.to_owned ();

	for index in 0 .. existing_line.len () {

		let sample_cell = sample_line [index];

		if solved_line [index] != UNKNOWN {
			continue;
		}

		proposed_line [index] = match sample_cell {
			EMPTY => FILLED,
			FILLED => EMPTY,
			_ => continue,
		};

		placed_clues.truncate (0);

		if ! place_clues (
			& mut placed_clues,
			& proposed_line,
			clues_line,
		) {
			proposed_line [index] = sample_cell;
			solved_line [index] = sample_cell;
			continue;
		}

		for (nested_index, placed_cell) in render_placed_clues (
			& placed_clues,
			existing_line.len (),
		).enumerate () {

			let nested_index = nested_index as LineSize;

			if placed_cell != sample_line [nested_index] {
				sample_line [nested_index] = UNKNOWN;
			}

		}

		proposed_line [index] = UNKNOWN;

	}

	Some (solved_line)

}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn test_place_clues_start_1 () {

		let mut result = Vec::new ();

		assert_eq! (
			place_clues (
				& mut result,
				& Line::from_str (" ----").unwrap (),
				& vec! [ 3 ],
			),
			true,
		);

		assert_eq! (
			result,
			vec! [ (1, 4) ],
		);

	}

	#[ test ]
	fn test_place_clues_start_2 () {

		let mut result = Vec::new ();

		assert_eq! (
			place_clues (
				& mut result,
				& Line::from_str ("----- ----").unwrap (),
				& vec! [ 3, 4 ],
			),
			true,
		);

		assert_eq! (
			result,
			vec! [ (0, 3), (6, 10) ],
		);

	}

	#[ test ]
	fn test_solve_line_1 () {

		assert_eq! (
			solve_line (
				& Line::from_str ("----------").unwrap (),
				& vec! [ 3, 2, 3 ],
			),
			Some (Line::from_str ("### ## ###").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_2 () {

		assert_eq! (
			solve_line (
				& Line::from_str ("----------").unwrap (),
				& vec! [ 3, 4 ],
			),
			Some (Line::from_str ("--#---##--").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_3 () {

		assert_eq! (
			solve_line (
				& Line::from_str ("----- ----").unwrap (),
				& vec! [ 3, 4 ],
			),
			Some (Line::from_str ("--#-- ####").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_4 () {

		assert_eq! (
			solve_line (
				& Line::from_str ("----# ----").unwrap (),
				& vec! [ 3, 4 ],
			),
			Some (Line::from_str ("  ### ####").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_5 () {

		assert_eq! (
			solve_line (
				& Line::from_str ("-#---#----").unwrap (),
				& vec! [ 3, 4 ],
			),
			Some (Line::from_str ("-##--###- ").unwrap ()),
		);

	}

	#[ test ]
	fn test_solve_line_6 () {

		assert_eq! (
			solve_line (
				& Line::from_str ("--- #-----").unwrap (),
				& vec! [ 2, 3 ],
			),
			Some (Line::from_str ("--- ##----").unwrap ()),
		);

	}

}

