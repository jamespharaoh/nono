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
	existing_line: & LineRef,
	clues_line: & [LineSize],
	offset: LineSize,
) -> Option <LineSize> {

	let result_len = result.len ();

	if clues_line.is_empty () {

		return if existing_line.iter ().all (
			|cell| * cell == UNKNOWN || * cell == EMPTY
		) {
			Some (0)
		} else {
			None
		}

	}

	let my_size = clues_line [0];

	if my_size > existing_line.len () {
		return None;
	}

	for my_start in 0 ..= existing_line.len () - my_size {

		if existing_line.iter ().skip (
			my_start as usize,
		).take (
			my_size as usize,
		).all (
			|cell| * cell == UNKNOWN || * cell == FILLED
		) && (false
			|| existing_line.len () == my_start + my_size
			|| existing_line [my_start + my_size] == UNKNOWN
			|| existing_line [my_start + my_size] == EMPTY
		) {

			if my_start + my_size == existing_line.len () {
				if clues_line.len () == 1 {
					result.push ((offset + my_start, offset + my_start + my_size));
					return Some (1);
				} else {
					return None;
				};
			}

			result.push ((offset + my_start, offset + my_start + my_size));

			if let Some (num) = place_clues_real (
				result,
				& existing_line [my_start + my_size + 1 .. existing_line.len ()],
				& clues_line [1 ..],
				offset + my_start + my_size + 1,
			) {

				return Some (1 + num);

			} else {

				result.truncate (result_len);

			}

		}

		if ! (false
			|| existing_line [my_start] == UNKNOWN
			|| existing_line [my_start] == EMPTY
		) {
			return None;
		}

	}

	None

}

fn place_clues (
	result: & mut Vec <(LineSize, LineSize)>,
	existing_line: & LineRef,
	clues_line: & [LineSize],
) -> Option <LineSize> {

	place_clues_real (
		result,
		existing_line,
		clues_line,
		0,
	)

}

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

	if place_clues (
		& mut placed_clues,
		existing_line,
		clues_line,
	).is_none () {
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

		if place_clues (
			& mut placed_clues,
			& proposed_line,
			clues_line,
		).is_none () {
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
			Some (1),
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
			Some (2),
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

