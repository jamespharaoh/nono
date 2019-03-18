#![ allow (unused_parens) ]

use std::cell::RefCell;
use std::env;
use std::io;
use std::path::Path;
use std::rc::Rc;
use std::thread;
use std::time;

use nono::*;

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

	if ! clues.is_consistent () {

		println! (
			"Inconsistent clues: row sum = {}, coll sum = {}",
			clues.rows_sum (),
			clues.cols_sum (),
		);

		return;

	}

	solve (& clues);

}

fn solve (
	clues: & Clues,
) {

	let grid = Rc::new (RefCell::new (
		Grid::new (
			clues.num_rows (),
			clues.num_cols (),
		),
	));

	let mut grid_printer = GridPrinter::new (& clues);

	grid_printer.print (
		& mut io::stdout ().lock (),
		& grid.borrow (),
	).unwrap ();

	let mut stats = SolveGridStats::new ();

	for event in solve_grid (
		grid.clone (),
		clues,
	) {

		let mut redraw = false;

		match event {

			SolveGridEvent::SolvedCell { .. } => {
				redraw = true;
			},

			SolveGridEvent::Row (index) => {
				print! ("\r\x1b[2Krow {} ...", index);
			},

			SolveGridEvent::Col (index) => {
				print! ("\r\x1b[2Kcol {} ...", index);
			},

			SolveGridEvent::SolvedGrid { .. } => {
				redraw = true;
			},

			SolveGridEvent::Stats (new_stats) => {
				stats = new_stats;
			},

			_ => (),

		};

		if redraw {

			thread::sleep (time::Duration::from_millis (20));

			grid_printer.print (
				& mut io::stdout ().lock (),
				& grid.borrow (),
			).unwrap ();

		}

	}

	println! (
		"\r\x1b[2KSolved in {} iterations, {} lines",
		stats.grid_iterations + 1,
		stats.line_iterations + 1,
	);

}

