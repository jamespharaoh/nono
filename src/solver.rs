use crate::clues::Clues;
use crate::grid::Grid;
use crate::line::LineSize;
use crate::line::solve_line;

pub fn solve_row (
	grid: & mut Grid,
	clues: & Clues,
	row_index: LineSize,
) -> bool {

	let existing_line = & grid.rows () [row_index];

	if existing_line.is_solved () {
		return false;
	}

	let combined_line = solve_line (
		& grid.rows () [row_index],
		& clues.rows [row_index],
		grid.num_cols (),
	);

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

	let existing_line = & grid.cols () [col_index];

	if existing_line.is_solved () {
		return false;
	}

	let combined_line = solve_line (
		& grid.cols () [col_index],
		& clues.cols [col_index],
		grid.num_rows (),
	);

	let mut progress = false;

	for row_index in 0 .. combined_line.len () {

		if grid.get (row_index, col_index) != combined_line [row_index] {
			progress = true;
		}

		grid.set (row_index, col_index, combined_line [row_index]);

	}

	progress

}

