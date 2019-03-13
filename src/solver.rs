use std::iter;

use crate::cell::UNKNOWN;
use crate::cell::EMPTY;
use crate::cell::FILLED;
use crate::clues::Clues;
use crate::clues::CluesLine;
use crate::grid::Grid;
use crate::line::Line;
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

pub fn line_fits (
	existing_line: & Line,
	candidate_line: & Line,
) -> bool {

	for (existing_cell, candidate_cell) in existing_line.iter ().zip (
		candidate_line.iter (),
	) {

		if (
			* existing_cell != UNKNOWN
			&& * existing_cell != * candidate_cell
		) {
			return false;
		}

	}

	true

}

fn place_clues_real (
	existing_line: & Line,
	clues_line: & [LineSize],
) -> Option <Vec <(LineSize, LineSize)>> {

	if clues_line.is_empty () {

		return if existing_line.iter ().all (
			|cell| * cell == UNKNOWN || * cell == EMPTY
		) {
			Some (vec! [])
		} else {
			None
		}

	}

	let size = clues_line [0];

	if size > existing_line.len () {
		return None;
	}

	for start in 0 ..= existing_line.len () - size {

		if existing_line.iter ().skip (
			start as usize,
		).take (
			size as usize,
		).all (
			|cell| * cell == UNKNOWN || * cell == FILLED
		) && (false
			|| existing_line.len () == start + size
			|| existing_line [start + size] == UNKNOWN
			|| existing_line [start + size] == EMPTY
		) {

			if start + size == existing_line.len () {
				return if clues_line.len () == 1 {
					Some (vec! [(start, start + size)])
				} else {
					None
				};
			}

			if let Some (rest) = place_clues_real (
				& existing_line [
					start + size + 1 .. existing_line.len ()
				].iter ().collect (),
				& clues_line [1 ..],
			) {

				return Some (
					vec! [ (start, start + size) ].into_iter ().chain (
						rest.iter ().map (
							|(nested_start, nested_end)| (
								nested_start + start + size + 1,
								nested_end + start + size + 1,
							)
						),
					).collect::<Vec <(LineSize, LineSize)>> (),
				);

			}

		}

		if ! (false
			|| existing_line [start] == UNKNOWN
			|| existing_line [start] == EMPTY
		) {
			return None;
		}

	}

	None

}

fn place_clues_start (
	existing_line: & Line,
	clues_line: & [LineSize],
) -> Option <Vec <LineSize>> {

	place_clues_real (
		existing_line,
		clues_line,
	).map (
		|clues| clues.iter ().map (
			|& (start, end)| start,
		).collect (),
	)

}

fn place_clues_end (
	existing_line: & Line,
	clues_line: & [LineSize],
) -> Option <Vec <LineSize>> {

	place_clues_real (
		& existing_line.iter ().rev ().cloned ().collect (),
		& clues_line.iter ().rev ().cloned ().collect::<Vec <_>> (),
	).map (
		|clues| clues.iter ().rev ().map (
			|& (start, end)| existing_line.len () - end,
		).collect (),
	)

}

#[ derive (Debug) ]
struct LineClue {
	index: usize,
	size: LineSize,
	min_start: LineSize,
	max_start: LineSize,
	min_end: LineSize,
	max_end: LineSize,
	starts: Vec <LineSize>,
}

fn line_clues (
	existing_line: & Line,
	clues_line: & CluesLine,
) -> Option <Vec <LineClue>> {

	let min_starts = match place_clues_start (
		& existing_line,
		& clues_line,
	) {
		Some (val) => val,
		None => return None,
	};

	let max_starts = match place_clues_end (
		& existing_line,
		& clues_line,
	) {
		Some (val) => val,
		None => return None,
	};

	Some (

		clues_line.iter ().cloned ().zip (
			min_starts.iter ().cloned ().zip (
				max_starts.iter ().cloned (),
			),
		).enumerate ().map (
			|(index, (size, (min_start, max_start)))| {

			LineClue {
				index: index,
				size: size,
				min_start: min_start,
				max_start: max_start,
				min_end: min_start + size,
				max_end: max_start + size,
				starts: (min_start ..= max_start).collect (),
			}

		}).collect (),

	)

}

pub fn solve_line (
	existing_line: & Line,
	clues_line: & CluesLine,
) -> Option <Line> {

	let mut solved_line = existing_line.clone ();

	let line_clues = match line_clues (
		existing_line,
		clues_line,
	) {
		Some (val) => val,
		None => return None,
	};

	let mut proposed_line = existing_line.clone ();

	for (cell_index, & existing_cell) in existing_line.iter ().enumerate () {

		let cell_index = cell_index as LineSize;

		if existing_cell != UNKNOWN {
			continue;
		}

		proposed_line [cell_index] = FILLED;

		if place_clues_start (
			& proposed_line,
			clues_line,
		).is_none () {
			solved_line [cell_index] = EMPTY;
		}

		proposed_line [cell_index] = EMPTY;

		if place_clues_start (
			& proposed_line,
			clues_line,
		).is_none () {
			solved_line [cell_index] = FILLED;
		}

		proposed_line [cell_index] = UNKNOWN;

	}

	Some (solved_line)

}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn test_place_clues_start_1 () {

		assert_eq! (
			place_clues_start (
				& Line::from_str (" ----").unwrap (),
				& vec! [ 3 ],
			),
			Some (vec! [ 1 ]),
		);

	}

	#[ test ]
	fn test_place_clues_start_2 () {

		assert_eq! (
			place_clues_start (
				& Line::from_str ("----- ----").unwrap (),
				& vec! [ 3, 4 ],
			),
			Some (vec! [ 0, 6 ]),
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

