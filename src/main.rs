#![ allow (unused_parens) ]

mod cell;
mod clues;
mod grid;
mod line;
mod samples;
mod solver;

use std::env;
use std::thread;
use std::time;

use crate::clues::Clues;
use crate::grid::Grid;
use crate::line::LineSize;
use crate::solver::solve_col;
use crate::solver::solve_row;

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

	println! ("Selected puzzle: {}", name);

	solve (& clues);

}

fn solve (
	clues: & Clues,
) {

	let mut grid = Grid::new (
		clues.rows.len () as LineSize,
		clues.cols.len () as LineSize,
	);

	let mut line_iterations = 0;
	let mut grid_iterations = 0;

	let mut vertical = false;
	let mut index = 0;

	grid.print ();

	while ! grid.is_solved () {

		// check if solved

		if grid.is_solved () {
			break;
		}

		if ! vertical && index == 0 {
			grid_iterations += 1;
		}

		// solve next

		let progress;

		if ! vertical {

			progress = solve_row (
				& mut grid,
				& clues,
				index,
			);

			index += 1;

			if index == grid.num_rows () {
				vertical = true;
				index = 0;
			}

		} else {

			progress = solve_col (
				& mut grid,
				& clues,
				index,
			);

			index += 1;

			if index == grid.num_cols () {
				vertical = false;
				index = 0;
			}

		}

		if progress {

			line_iterations += 1;

			thread::sleep (time::Duration::from_millis (100));

			print! ("\x1b[{}A", grid.num_rows () + 2);

			grid.print ();

		}

	}

	println! (
		"Solved in {} iterations, {} lines",
		grid_iterations,
		line_iterations);

}

