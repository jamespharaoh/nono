#![ allow (unused_parens) ]

use std::env;
use std::io;
use std::path::Path;
use std::thread;
use std::time;

use nono::*;

fn main () {

	// parse args

	let args: Vec <String> = env::args ().collect ();

	if args.len () != 2 {
		println! ("Syntax: nono-cli FILE");
		return;
	}

	let name = & args [1];

	// load clues

	let clues = Clues::load_file (
		& Path::new (name),
	).unwrap ();

	if ! clues.is_consistent () {

		println! (
			"Inconsistent clues: row sum = {}, coll sum = {}",
			clues.rows_sum (),
			clues.cols_sum (),
		);

		return;

	}

	// solve

	solve (clues);

}

fn solve (
	clues: Clues,
) {

	let grid = Grid::new (
		clues.num_rows (),
		clues.num_cols (),
	);

	let mut grid_printer = GridPrinter::new (& clues);

	grid_printer.print (
		& mut io::stdout ().lock (),
		& grid,
	).unwrap ();

	let mut grid_solver = GridSolver::new (
		grid,
		clues,
	);

	while let Some (event) = grid_solver.next () {

		let mut redraw = false;

		match event {

			GridSolverEvent::SolvedCell { .. } => {
				redraw = true;
			},

			GridSolverEvent::StartRow (index) => {
				print! ("\r\x1b[2Krow {} ...", index);
			},

			GridSolverEvent::StartCol (index) => {
				print! ("\r\x1b[2Kcol {} ...", index);
			},

			GridSolverEvent::SolvedGrid { .. } => {
				redraw = true;
			},

			_ => (),

		};

		if redraw {

			grid_printer.print (
				& mut io::stdout ().lock (),
				& grid_solver.grid (),
			).unwrap ();

			thread::sleep (time::Duration::from_millis (20));

		}

	}

	println! (
		"\r\x1b[2KSolved in {} iterations, {} lines",
		grid_solver.stats ().grid_iterations,
		grid_solver.stats ().line_iterations,
	);

}

