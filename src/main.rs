#![ allow (unused_parens) ]

mod cell;
mod clues;
mod grid;
mod line;
mod samples;

use std::env;

use crate::grid::Grid;

use crate::line::Line;
use crate::line::LineSize;
use crate::line::solve_line;

use crate::clues::Clues;

fn main () {

	let args: Vec <String> = env::args ().collect ();

	let name = if args.len () >= 2 {
		& args [1]
	} else {
		""
	};

	let clues = match name {

		"camel" => samples::camel (),
		"eagle" => samples::eagle (),

		_ => {
			println! ("No such sample: {}", name);
			return;
		}

	};

	println! ("N: [{}]", name);

	solve (& clues);

}

fn solve (
	clues: & Clues,
) {

	let mut grid = Grid::new (
		clues.rows.len () as LineSize,
		clues.cols.len () as LineSize,
	);

	let mut iterations = 0;

	while ! grid.is_solved () {

		solve_rows (
			& mut grid.rows,
			& mut grid.cols,
			& clues.rows,
		);

		iterations += 1;

		grid.print ();

		if grid.is_solved () {
			break;
		}

		solve_rows (
			& mut grid.cols,
			& mut grid.rows,
			& clues.cols,
		);

		iterations += 1;

		grid.print ();

	}

	println! ("Solved in {} iterations", iterations);

}

fn solve_rows (
	grid_rows: & mut Vec <Line>,
	grid_cols: & mut Vec <Line>,
	all_clues: & Vec <Vec <u16>>,
) {

	for row_num in 0 .. grid_rows.len () {

		let existing_line = & grid_rows [row_num];

		if existing_line.is_solved () {
			continue;
		}

		let combined_line = solve_line (
			& grid_rows [row_num],
			& all_clues [row_num],
			grid_cols.len () as LineSize,
		);

		for col_num in 0 .. combined_line.len () {
			grid_rows [row_num as usize] [col_num as LineSize] = combined_line [col_num];
			grid_cols [col_num as usize] [row_num as LineSize] = combined_line [col_num];
		}

	}

}

