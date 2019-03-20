#![ allow (unused_parens) ]

extern crate cairo;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::cell::RefCell;
use std::env;
use std::io;
use std::rc::Rc;

use nono::*;

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
	solver: Rc <RefCell <GridSolver>>,
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
			solver: Rc::new (RefCell::new (solver)),
		};

		solver_window.build_ui (application);

		solver_window

	}

	fn build_ui (
		& self,
		application: & gtk::Application,
	) {

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

		let self_clone = self.clone ();
		gtk::timeout_add (10, move || self_clone.tick (& window));

	}

	fn tick (& self, window: & gtk::ApplicationWindow) -> gtk::Continue {

		let mut solver = self.solver.borrow_mut ();

		let _event = match solver.next () {
			Some (val) => val,
			None => return gtk::Continue (false),
		};

		window.queue_draw ();

		gtk::Continue (true)

	}

	fn draw_fn (
		& self,
		drawing_area: & gtk::DrawingArea,
		context: & cairo::Context,
	) -> gtk::Inhibit {

		let background = cairo::Pattern::SolidPattern (
			cairo::SolidPattern::from_rgb (0.85, 0.85, 0.85));

		let lines = cairo::Pattern::SolidPattern (
			cairo::SolidPattern::from_rgb (0.00, 0.00, 0.00));

		let unknown = cairo::Pattern::SolidPattern (
			cairo::SolidPattern::from_rgb (0.70, 0.70, 0.70));

		let filled = cairo::Pattern::SolidPattern (
			cairo::SolidPattern::from_rgb (0.10, 0.10, 0.10));

		let empty = cairo::Pattern::SolidPattern (
			cairo::SolidPattern::from_rgb (1.00, 1.00, 1.00));

		let error = cairo::Pattern::SolidPattern (
			cairo::SolidPattern::from_rgb (0.80, 0.20, 0.20));

		let solver = self.solver.borrow ();
		let grid = solver.grid ();

		// grid dimensions

		let border_size = 1.0 as f64;
		let cell_size = 1.0 as f64;
		let thick_line_size = 0.15 as f64;
		let thin_line_size = 0.05 as f64;

		let grid_left = border_size;
		let grid_top = border_size;
		let grid_width = cell_size * grid.num_cols () as f64;
		let grid_height = cell_size * grid.num_rows () as f64;

		let total_width = border_size * 2.0 + grid_width;
		let total_height = border_size * 2.0 + grid_height;
		let total_ratio = total_width / total_height;

		// position and scale

		let native_width = drawing_area.get_allocated_width () as f64;
		let native_height = drawing_area.get_allocated_height () as f64;
		let native_ratio = native_width / native_height;

		let scale = if native_ratio > total_ratio {
			native_height / total_height
		} else {
			native_width / total_width
		};

		context.translate (
			(native_width - total_width * scale) / 2.0,
			(native_height - total_height * scale) / 2.0,
		);

		context.scale (scale, scale);

		// background

		context.set_source (& background);
		context.paint ();

		// grid cells

		for row_index in 0 .. grid.num_rows () {
			for col_index in 0 .. grid.num_cols () {

				match grid [(row_index, col_index)] {
					Cell::UNKNOWN => context.set_source (& unknown),
					Cell::EMPTY => context.set_source (& empty),
					Cell::FILLED => context.set_source (& filled),
					_ => context.set_source (& error),
				};

				context.rectangle (
					grid_left + col_index as f64,
					grid_top + row_index as f64,
					cell_size,
					cell_size,
				);

				context.fill ();

			}

		}

		// grid lines

		context.set_source (& lines);
		context.set_line_cap (cairo::LineCap::Square);

		for row_index in 0 ..= grid.num_rows () {

			context.set_line_width (
				if row_index % 5 == 0 || row_index == grid.num_rows () {
					thick_line_size
				} else {
					thin_line_size
				},
			);

			context.move_to (grid_left, grid_top + cell_size * row_index as f64);
			context.rel_line_to (grid_width, 0.0);
			context.stroke ();

		}

		for col_index in 0 ..= grid.num_cols () {

			context.set_line_width (
				if col_index % 5 == 0 || col_index == grid.num_cols () {
					thick_line_size
				} else {
					thin_line_size
				},
			);

			context.move_to (grid_left + cell_size * col_index as f64, grid_top);
			context.rel_line_to (0.0, grid_height);
			//context.close_path ();
			context.stroke ();

		}

		gtk::Inhibit (false)

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

