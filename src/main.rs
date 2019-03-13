#![ allow (unused_parens) ]

mod cell;
mod clues;
mod grid;
mod line;
mod solver;

use std::env;
use std::io;
use std::io::Write;
use std::iter;
use std::path::Path;
use std::thread;
use std::time;

use crate::cell::EMPTY;
use crate::cell::ERROR;
use crate::cell::FILLED;
use crate::cell::UNKNOWN;
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

	let clues = Clues::load (
		& Path::new (name),
	).unwrap ();

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

	let grid_size = print_grid (& grid, & clues);

	while ! grid.is_solved () {

		// check if solved

		if grid.is_solved () {
			break;
		}

		if ! vertical && index == 0 {
			grid_iterations += 1;
		}

		print! ("\r\x1b[2K{} {} ...", if vertical { "col" } else { "row" }, index);

		io::stdout ().flush ().unwrap ();

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

			print! ("\r\x1b[{}A", grid_size);

			print_grid (& grid, & clues);

		}

	}

	println! (
		"\r\x1b[2KSolved in {} iterations, {} lines",
		grid_iterations,
		line_iterations);

}

pub fn print_grid (
	grid: & Grid,
	clues: & Clues,
) -> usize {

	let max_row_clues = clues.rows.iter ().map (
		|clues_line| clues_line.len (),
	).max ().unwrap_or (0);

	let max_col_clues = clues.cols.iter ().map (
		|clues_line| clues_line.len (),
	).max ().unwrap_or (0);

	for row in 0 .. max_col_clues {

		print! ("{} ", iter::repeat ("  ").take (max_row_clues).collect::<String> ());

		for col in 0 .. grid.num_cols () {
			if (max_col_clues - row - 1) < clues.cols [col as usize].len () {
				print! (
					"{:2}",
					clues.cols [col as usize] [
						row - (max_col_clues - clues.cols [col as usize].len ())
					]);
			} else {
				print! ("  ");
			}
		}

		print! ("\n");

	}

	println! (
		"{}┌{}┐",
		iter::repeat ("  ").take (max_row_clues).collect::<String> (),
		iter::repeat ("─").take (
			grid.num_cols () as usize * 2,
		).collect::<String> ());

	for (row_index, row) in grid.rows ().iter ().enumerate () {

		print! (
			"{}",
			iter::repeat ("  ").take (
				max_row_clues - clues.rows [row_index].len (),
			).collect::<String> (),
		);

		for clue in clues.rows [row_index].iter () {
			print! ("{:2}", clue);
		}

		println! (
			"│{}│",
			row.iter ().map (
				|cell|

				match * cell {
					UNKNOWN => "░░",
					EMPTY => "  ",
					FILLED => "██",
					ERROR => "!!",
					_ => "??",
				}

			).collect::<String> ());

	}

	println! (
		"{}└{}┘",
		iter::repeat ("  ").take (max_row_clues).collect::<String> (),
		iter::repeat ("─").take (
			grid.num_cols () as usize * 2,
		).collect::<String> ());

	max_col_clues + 1 + grid.num_rows () as usize + 1

}

