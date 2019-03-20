use std::iter;
use std::ops;

use crate::data::*;
use crate::solver::*;

pub struct GridSolver {

	grid: Grid,
	clues: Clues,

	stats: GridSolverStats,
	changed_rows: Vec <bool>,
	changed_cols: Vec <bool>,

	vertical: bool,
	index: LineSize,
	index_changed: bool,
	line_solver: Option <iter::Zip <ops::RangeFrom <LineSize>, LineSolver>>,
	complete: bool,

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

			vertical: false,
			index: 0,
			index_changed: true,
			line_solver: None,
			complete: false,

		}

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
	) -> Box <Iterator <Item = & 'a Cell> + 'a> {

		if ! self.vertical {
			Box::new (self.grid.row (self.index))
		} else {
			Box::new (self.grid.col (self.index))
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

	pub fn next (
		& mut self,
	) -> Option <GridSolverEvent> {

		if self.complete {
			return None;
		}

		loop {

			if self.line_solver.is_some () {

				if let Some ((cell_index, cell)) =
					self.line_solver.as_mut ().unwrap ().next () {

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

				self.line_solver = None;
				self.unset_line_changed ();
				self.stats.line_iterations += 1;

				continue;

			}

			if self.grid.is_solved () {
				self.stats.grid_iterations += 1;
				self.complete = true;
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

			let existing_line = self.get_line ().collect::<LineBuf> ();

			self.line_solver = Some (
				Iterator::zip (
					ops::RangeFrom { start: 0 },
					solve_line (
						existing_line,
						self.get_clues ().to_owned (),
					).unwrap (),
				),
			);

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

