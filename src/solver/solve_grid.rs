use std::cell::RefCell;
use std::iter;
use std::ops;
use std::rc::Rc;

use crate::data::*;
use crate::solver::*;

pub fn solve_grid <'a> (
	grid: Rc <RefCell <Grid>>,
	clues: & 'a Clues,
) -> SolveGridIter <'a> {

	SolveGridIter::new (
		grid,
		clues,
	)

}

pub struct SolveGridIter <'a> {
	grid: Rc <RefCell <Grid>>,
	num_rows: LineSize,
	num_cols: LineSize,
	clues: & 'a Clues,
	index: LineSize,
	vertical: bool,
	stats: SolveGridStats,
	line_solver: Option <iter::Zip <ops::RangeFrom <LineSize>, LineSolver>>,
	index_changed: bool,
	stats_changed: bool,
	complete: bool,
}

#[ derive (Clone, Copy, Debug) ]
pub struct SolveGridStats {
	pub grid_iterations: usize,
	pub line_iterations: usize,
}

#[ derive (Debug) ]
pub enum SolveGridEvent {

	Row (LineSize),
	Col (LineSize),

	SolvedCell {
		row_index: LineSize,
		col_index: LineSize,
		value: Cell,
	},

	SolvedRow (LineSize),
	SolvedGrid,

	Stats (SolveGridStats),

}

impl <'a> SolveGridIter <'a> {

	pub fn new (
		grid: Rc <RefCell <Grid>>,
		clues: & 'a Clues,
	) -> SolveGridIter <'a> {

		let grid_value = grid.borrow ();

		let num_rows = grid_value.num_rows ();
		let num_cols = grid_value.num_cols ();

		SolveGridIter {
			grid: grid.clone (),
			num_rows: num_rows,
			num_cols: num_cols,
			clues: clues,
			index: 0,
			vertical: false,
			stats: SolveGridStats::new (),
			line_solver: None,
			index_changed: true,
			stats_changed: true,
			complete: false,
		}

	}

	fn advance (& mut self) {

		let max_index = if ! self.vertical {
			self.num_rows
		} else {
			self.num_cols
		};

		self.index += 1;

		if self.index == max_index {

			if self.vertical {
				self.stats.grid_iterations += 1;
				self.stats_changed = true;
			}

			self.vertical = ! self.vertical;
			self.index = 0;

		}

		self.index_changed = true;

	}

	fn get_clues (& self) -> & CluesLine {

		if ! self.vertical {
			self.clues.row (self.index)
		} else {
			self.clues.col (self.index)
		}

	}

	fn get_line <'b> (
		& self,
		grid: & 'b Grid,
	) -> impl Iterator <Item = & 'b Cell> + 'b {

		let index = self.index;
		let vertical = self.vertical;

		let mut iter = if ! vertical {
			Box::new (grid.row (index)) as Box <Iterator <Item = & 'b Cell> + 'b>
		} else {
			Box::new (grid.col (index)) as Box <Iterator <Item = & 'b Cell> + 'b>
		};

		iter::repeat_with (
			move || iter.next (),
		).take_while (
			Option::is_some,
		).map (
			Option::unwrap,
		)

	}

	fn get_cell (& mut self, cell_index: LineSize) -> Cell {

		if ! self.vertical {
			self.grid.borrow () [(self.index, cell_index)]
		} else {
			self.grid.borrow () [(cell_index, self.index)]
		}

	}

	fn set_cell (& mut self, cell_index: LineSize, cell: Cell) {

		if ! self.vertical {
			self.grid.borrow_mut () [(self.index, cell_index)] = cell
		} else {
			self.grid.borrow_mut () [(cell_index, self.index)] = cell
		}

	}

}

impl <'a> Iterator for SolveGridIter <'a> {

	type Item = SolveGridEvent;

	fn next (
		& mut self,
	) -> Option <SolveGridEvent> {

		loop {

			if self.complete {
				return None;
			}

			if self.stats_changed {
				self.stats_changed = false;
				return Some (SolveGridEvent::Stats (self.stats));
			}

			if self.line_solver.is_some () {

				if let Some ((cell_index, cell)) =
					self.line_solver.as_mut ().unwrap ().next () {

					if cell == self.get_cell (cell_index) {
						continue;
					}

					self.set_cell (cell_index, cell);

					return Some (
						if ! self.vertical {

							SolveGridEvent::SolvedCell {
								row_index: self.index,
								col_index: cell_index,
								value: cell,
							}

						} else {

							SolveGridEvent::SolvedCell {
								row_index: cell_index,
								col_index: self.index,
								value: cell,
							}

						}
					);

				}

				self.line_solver = None;

				self.advance ();

				self.stats.line_iterations += 1;
				self.stats_changed = true;

				continue;

			}

			if self.grid.borrow ().is_solved () {
				self.complete = true;
				return Some (SolveGridEvent::SolvedGrid);
			}

			while self.get_line (
				& self.grid.borrow (),
			).all (Cell::is_solved) {
				self.advance ();
			}

			if self.index_changed {

				self.index_changed = false;

				return Some (
					if ! self.vertical {
						SolveGridEvent::Row (self.index)
					} else {
						SolveGridEvent::Col (self.index)
					}
				);

			}

			let existing_line = self.get_line (
				& self.grid.borrow (),
			).collect::<LineBuf> ();

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

}

impl SolveGridStats {

	pub fn new () -> SolveGridStats {

		SolveGridStats {
			grid_iterations: 0,
			line_iterations: 0,
		}

	}

}

