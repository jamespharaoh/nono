use std::alloc::System;
use std::env;
use std::mem;
use std::path::Path;

use nono::*;

use stats_alloc::INSTRUMENTED_SYSTEM;
use stats_alloc::Region;
use stats_alloc::StatsAlloc;

#[global_allocator]
static GLOBAL: & StatsAlloc <System> = & INSTRUMENTED_SYSTEM;

fn main () {

	let args: Vec <String> = env::args ().collect ();
	let clues = Clues::load_file (& Path::new (& args [1])).unwrap ();

	let reg = Region::new (& GLOBAL);

	let clues = solve (clues);

	println! ("{:#?}", reg.change ());

	mem::drop (clues);

}

fn solve (clues: Clues) -> Clues {

	let grid = Grid::new (clues.num_rows (), clues.num_cols ());

	let mut grid_solver = GridSolver::new (grid, clues);

	while let Some (_event) = grid_solver.next () {
		//println! ("{:?}", event);
		//println! ("{:#?}", reg.change ());
	}

	let (clues, _grid) = grid_solver.release ();

	clues

}

