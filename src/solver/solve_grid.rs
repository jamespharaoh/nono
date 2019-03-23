use std::iter;
use std::mem;

use crate::data::*;
use crate::solver::*;

pub struct GridSolver {

	grid: Grid,
	clues: Clues,

	stats: GridSolverStats,
	changed_rows: Vec <bool>,
	changed_cols: Vec <bool>,
	line_solver: LineSolver,

	state: State,
	vertical: bool,
	index: LineSize,
	index_changed: bool,

}

enum State {
	Scanning,
	Solving (LineSize),
	Complete,
}

#[ derive (Clone, Copy, Debug) ]
pub struct GridSolverStats {
	pub grid_iterations: usize,
	pub line_iterations: usize,
}

#[ derive (Debug) ]
pub enum GridSolverEvent {

	StartRow (LineSize),
	StartCol (LineSize),

	SolvedCell (LineSize, LineSize),
	SolvedRow (LineSize),
	SolvedCol (LineSize),
	SolvedGrid,

}

impl GridSolver {

	pub fn new (
		grid: Grid,
		clues: Clues,
	) -> GridSolver {

		let changed_rows = iter::repeat (true).take (
			grid.num_rows () as usize,
		).collect ();

		let changed_cols = iter::repeat (true).take (
			grid.num_cols () as usize,
		).collect ();

		GridSolver {

			grid: grid,
			clues: clues,

			stats: GridSolverStats::new (),
			changed_rows: changed_rows,
			changed_cols: changed_cols,
			line_solver: Default::default (),

			vertical: false,
			index: 0,
			index_changed: true,
			state: State::Scanning,

		}

	}

	pub fn release (self) -> (Clues, Grid) {
		(self.clues, self.grid)
	}

	pub fn clues (& self) -> & Clues {
		& self.clues
	}

	pub fn grid (& self) -> & Grid {
		& self.grid
	}

	pub fn stats (& self) -> & GridSolverStats {
		& self.stats
	}

	fn advance (& mut self) {

		let max_index = if ! self.vertical {
			self.grid.num_rows ()
		} else {
			self.grid.num_cols ()
		};

		if ! self.vertical {
			self.changed_rows [self.index as usize] = false;
		} else {
			self.changed_cols [self.index as usize] = false;
		}

		self.index += 1;

		if self.index == max_index {

			if self.vertical {
				self.stats.grid_iterations += 1;
			}

			self.vertical = ! self.vertical;
			self.index = 0;

		}

		self.index_changed = true;

	}

	fn get_line_changed (& self) -> bool {

		if ! self.vertical {
			self.changed_rows [self.index as usize]
		} else {
			self.changed_cols [self.index as usize]
		}

	}

	fn unset_line_changed (& mut self) {

		if ! self.vertical {
			self.changed_rows [self.index as usize] = false
		} else {
			self.changed_cols [self.index as usize] = false
		}

	}

	fn get_clues (& self) -> & CluesLine {

		if ! self.vertical {
			self.clues.row (self.index)
		} else {
			self.clues.col (self.index)
		}

	}

	fn get_line <'a> (
		& 'a self,
	) -> GridIter <'a> {

		if ! self.vertical {
			self.grid.row (self.index)
		} else {
			self.grid.col (self.index)
		}

	}

	fn get_cell (& mut self, cell_index: LineSize) -> Cell {

		if ! self.vertical {
			self.grid [(self.index, cell_index)]
		} else {
			self.grid [(cell_index, self.index)]
		}

	}

	fn set_cell (& mut self, cell_index: LineSize, cell: Cell) {

		if ! self.vertical {
			self.grid [(self.index, cell_index)] = cell;
			self.changed_cols [cell_index as usize] = true
		} else {
			self.grid [(cell_index, self.index)] = cell;
			self.changed_rows [cell_index as usize] = true;
		}

	}

	fn is_complete (& self) -> bool {
		match self.state {
			State::Complete => true,
			_ => false,
		}
	}

	fn is_solving (& self) -> bool {
		match self.state {
			State::Solving (..) => true,
			_ => false,
		}
	}

	fn next_cell (& mut self) -> Option <(LineSize, Cell)> {

		match self.state {

			State::Solving (ref mut cell_index) => {

				if let Some (cell) = self.line_solver.next () {

					let result = (* cell_index, cell);
					* cell_index += 1;
					Some (result)

				} else {
					None
				}

			},

			_ => panic! (),

		}

	}

	pub fn next (
		& mut self,
	) -> Option <GridSolverEvent> {

		if self.is_complete () {
			return None;
		}

		loop {

			if self.is_solving () {

				if let Some ((cell_index, cell)) = self.next_cell () {

					if cell == self.get_cell (cell_index) {
						continue;
					}

					self.set_cell (cell_index, cell);

					return Some (
						if ! self.vertical {
							GridSolverEvent::SolvedCell (self.index, cell_index)
						} else {
							GridSolverEvent::SolvedCell (cell_index, self.index)
						}
					);

				}

				self.state = State::Scanning;
				self.unset_line_changed ();
				self.stats.line_iterations += 1;

				continue;

			}

			if self.grid.is_solved () {
				self.stats.grid_iterations += 1;
				self.state = State::Complete;
				return Some (GridSolverEvent::SolvedGrid);
			}

			while ! self.get_line_changed ()
			|| self.get_line ().all (Cell::is_solved) {
				self.advance ();
			}

			if self.index_changed {

				self.index_changed = false;

				return Some (
					if ! self.vertical {
						GridSolverEvent::StartRow (self.index)
					} else {
						GridSolverEvent::StartCol (self.index)
					}
				);

			}

			self.state = State::Solving (0);

			let mut line_solver = Default::default ();
			mem::swap (& mut line_solver, & mut self.line_solver);

			self.line_solver = match line_solver.into_new (
				self.get_line (),
				self.get_clues (),
			) {
				Ok (val) => val,
				Err (_) => panic! (),
			};

		}

	}

	pub fn end (self) -> (Grid, GridSolverStats) {
		(self.grid, self.stats)
	}

}

impl GridSolverStats {

	pub fn new () -> GridSolverStats {

		GridSolverStats {
			grid_iterations: 0,
			line_iterations: 0,
		}

	}

}

