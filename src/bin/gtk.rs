#![ allow (unused_parens) ]

extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::cell::RefCell;
use std::env;
use std::io;
use std::rc::Rc;

use nono::*;

const BORDER_SIZE: f64 = 20.5;
const CELL_SIZE: f64 = 20.0;
const THICK_LINE_SIZE: f64 = 3.0;
const THIN_LINE_SIZE: f64 = 1.0;

fn main () {

	// create gtk app

	let application = gtk::Application::new (
		"com.jamespharaoh.nono",
		gio::ApplicationFlags::HANDLES_OPEN,
	).expect ("Initialization failed...");

	application.connect_open (|app, files, hint| {
		handle_open (app, files, hint);
	});

	application.run (
		& env::args ().collect::<Vec <_>> (),
	);

}

fn handle_open (
	application: & gtk::Application,
	files: & [gio::File ],
	_hint: & str,
) {

	for file in files {
		handle_open_one (application, file);
	}

}

fn handle_open_one (
	application: & gtk::Application,
	file: & gio::File,
) {

	// load clues

	let file_input_stream = file.read (gio::NONE_CANCELLABLE).unwrap ();

	let mut reader = InputStreamReader {
		input_stream: file_input_stream.upcast (),
	};

	let clues = Clues::load (& mut reader).unwrap ();

	if ! clues.is_consistent () {

		println! (
			"Inconsistent clues: row sum = {}, coll sum = {}",
			clues.rows_sum (),
			clues.cols_sum (),
		);

		return;

	}

	SolverWindow::new (
		application,
		clues,
	);

}

#[ derive (Clone) ]
struct SolverWindow {
	state: Rc <RefCell <SolverWindowState>>,
}

struct SolverWindowState {
	solver: GridSolver,
	palette: Palette,
	grid_image_width: i32,
	grid_image_height: i32,
}

impl SolverWindow {

	pub fn new (
		application: & gtk::Application,
		clues: Clues,
	) -> SolverWindow {

		let solver = GridSolver::new (
			Grid::new (clues.num_rows (), clues.num_cols ()),
			clues,
		);

		let solver_window = SolverWindow {
			state: Rc::new (RefCell::new (SolverWindowState {
				solver: solver,
				palette: Palette::new (),
				grid_image_width: 0,
				grid_image_height: 0,
			})),
		};

		solver_window.build_ui (application);

		solver_window

	}

	fn build_ui (
		& self,
		application: & gtk::Application,
	) {

		let mut state = self.state.borrow_mut ();

		let window = gtk::ApplicationWindow::new (application);
		window.set_title ("Nono solver");
		window.set_default_size (500, 500);

		let drawing_area = Box::new (gtk::DrawingArea::new) ();
		let self_clone = self.clone ();
		drawing_area.connect_draw (move |drawing_area, context|
			self_clone.draw_fn (drawing_area, context),
		);
		window.add (& drawing_area);

		window.show_all ();

		state.grid_image_width = (0.0
			+ CELL_SIZE * state.solver.grid ().num_cols () as f64
			+ BORDER_SIZE * 2.0
		) as i32;

		state.grid_image_height = (0.0
			+ CELL_SIZE * state.solver.grid ().num_rows () as f64
			+ BORDER_SIZE * 2.0
		) as i32;

		let self_clone = self.clone ();
		gtk::timeout_add (10, move || self_clone.tick (& window));

	}

	fn tick (& self, window: & gtk::ApplicationWindow) -> gtk::Continue {

		if ! self.solve_one_cell () {
			return gtk::Continue (false);
		}

		window.queue_draw ();

		gtk::Continue (true)

	}

	fn solve_one_cell (& self) -> bool {

		let mut state = self.state.borrow_mut ();

		let _event = match state.solver.next () {
			Some (val) => val,
			None => return false,
		};

		true

	}

	fn draw_fn (
		& self,
		drawing_area: & gtk::DrawingArea,
		context: & cairo::Context,
	) -> gtk::Inhibit {

		let state = self.state.borrow ();

		// background

		context.set_source (& state.palette.background);
		context.paint ();

		// grid

		let image_width = state.grid_image_width as f64;
		let image_height = state.grid_image_height as f64;
		let image_ratio = image_width / image_height;

		let native_width = drawing_area.get_allocated_width () as f64;
		let native_height = drawing_area.get_allocated_height () as f64;
		let native_ratio = native_width / native_height;

		let scale = if native_ratio > image_ratio {
			native_height / image_height
		} else {
			native_width / image_width
		};

		context.translate (
			(native_width - image_width * scale) / 2.0,
			(native_height - image_height * scale) / 2.0,
		);

		context.scale (scale, scale);

		Self::draw_grid (& state, & context);

		// return

		gtk::Inhibit (false)

	}

	fn draw_grid (
		state: & SolverWindowState,
		context: & cairo::Context,
	) {

		let grid = state.solver.grid ();
		let palette = & state.palette;

		// grid dimensions

		let grid_left = BORDER_SIZE;
		let grid_top = BORDER_SIZE;
		let grid_width = CELL_SIZE * grid.num_cols () as f64;
		let grid_height = CELL_SIZE * grid.num_rows () as f64;

		// background

		context.set_source (& palette.background);
		context.paint ();

		// grid cells

		context.set_antialias (cairo::Antialias::None);

		for row_index in 0 .. grid.num_rows () {
			for col_index in 0 .. grid.num_cols () {

				context.set_source (
					match grid [(row_index, col_index)] {
						Cell::UNKNOWN => & palette.unknown,
						Cell::EMPTY   => & palette.empty,
						Cell::FILLED  => & palette.filled,
						_             => & palette.error,
					},
				);

				context.rectangle (
					grid_left + CELL_SIZE * col_index as f64,
					grid_top + CELL_SIZE * row_index as f64,
					CELL_SIZE,
					CELL_SIZE,
				);

				context.fill ();

			}

		}

		// grid lines

		context.set_antialias (cairo::Antialias::Default);
		context.set_source (& palette.lines);
		context.set_line_cap (cairo::LineCap::Square);

		for row_index in 0 ..= grid.num_rows () {

			context.set_line_width (
				if row_index % 5 == 0 || row_index == grid.num_rows () {
					THICK_LINE_SIZE
				} else {
					THIN_LINE_SIZE
				},
			);

			context.move_to (grid_left, grid_top + CELL_SIZE * row_index as f64);
			context.rel_line_to (grid_width, 0.0);
			context.stroke ();

		}

		for col_index in 0 ..= grid.num_cols () {

			context.set_line_width (
				if col_index % 5 == 0 || col_index == grid.num_cols () {
					THICK_LINE_SIZE
				} else {
					THIN_LINE_SIZE
				},
			);

			context.move_to (grid_left + CELL_SIZE * col_index as f64, grid_top);
			context.rel_line_to (0.0, grid_height);
			context.stroke ();

		}

	}

}

struct InputStreamReader {
	input_stream: gio::InputStream,
}

impl io::Read for InputStreamReader {

	fn read (
		& mut self,
		buffer: & mut [u8],
	) -> Result <usize, io::Error> {

		let bytes = self.input_stream.read_bytes (
			buffer.len (),
			gio::NONE_CANCELLABLE,
		).map_err (|gio_err|
			io::Error::new (
				io::ErrorKind::Other,
				gio_err,
			)
		) ?;

		buffer [0 .. bytes.len ()].copy_from_slice (& bytes);

		Ok (bytes.len ())

	}

}

struct Palette {
	background: cairo::Pattern,
	lines: cairo::Pattern,
	unknown: cairo::Pattern,
	filled: cairo::Pattern,
	empty: cairo::Pattern,
	error: cairo::Pattern,
}

impl Palette {

	fn new () -> Palette {

		Palette {
			background: Self::from_rgb (0.85, 0.85, 0.85),
			lines:      Self::from_rgb (0.00, 0.00, 0.00),
			unknown:    Self::from_rgb (0.70, 0.70, 0.70),
			filled:     Self::from_rgb (0.10, 0.10, 0.10),
			empty:      Self::from_rgb (1.00, 1.00, 1.00),
			error:      Self::from_rgb (0.80, 0.20, 0.20),
		}

	}

	fn from_rgb (red: f64, green: f64, blue: f64) -> cairo::Pattern {

		cairo::Pattern::SolidPattern (
			cairo::SolidPattern::from_rgb (red, green, blue),
		)

	}

}

