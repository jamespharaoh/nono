#![ allow (unused_parens) ]

extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::SettingsExt;

use std::cell::RefCell;
use std::env;
use std::io;
use std::rc::Rc;

use nono::*;

const BORDER_SIZE: f64 = 20.0;
const CELL_SIZE: f64 = 20.0;
const THICK_LINE_SIZE: f64 = 3.0;
const THIN_LINE_SIZE: f64 = 1.0;
const CLUE_FONT_SIZE: f64 = 14.0;
const CLUE_GAP: f64 = 2.0;

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
	dimensions: SolverWindowDimensions,
	window: Option <gtk::ApplicationWindow>,
	timeout_source: Option <glib::source::SourceId>,
}

#[ derive (Default) ]
struct SolverWindowDimensions {
	size: Size,
	grid: Rectangle,
	row_clues: Rectangle,
	col_clues: Rectangle,
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
				dimensions: Default::default (),
				window: None,
				timeout_source: None,
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
		state.window = Some (window.clone ());
		window.set_title ("Nono solver");
		window.set_default_size (500, 500);

		let drawing_area = Box::new (gtk::DrawingArea::new) ();
		let self_clone = self.clone ();
		drawing_area.connect_draw (move |drawing_area, context|
			self_clone.draw_fn (drawing_area, context),
		);
		window.add (& drawing_area);

		window.show_all ();

		state.dimensions = Self::calculate_dimensions (
			& state.solver.clues (),
			& state.solver.grid (),
			& drawing_area,
		);

		let self_clone = self.clone ();
		state.timeout_source = Some (
			gtk::timeout_add (10, move || self_clone.tick ()),
		);

		let self_clone = self.clone ();
		window.connect_destroy (move |_window|
			self_clone.destroy (),
		);

	}

	fn calculate_dimensions (
		clues: & Clues,
		grid: & Grid,
		drawing_area: & gtk::DrawingArea,
	) -> SolverWindowDimensions {

		// sizes

		let max_row_clues = clues.rows ().map (Vec::len).max ().unwrap_or (0);
		let max_col_clues = clues.cols ().map (Vec::len).max ().unwrap_or (0);

		let grid_size = Size {
			width: CELL_SIZE * grid.num_cols () as f64 + THICK_LINE_SIZE,
			height: CELL_SIZE * grid.num_rows () as f64 + THICK_LINE_SIZE,
		};

		let row_clues_size = Size {
			width: CELL_SIZE * max_row_clues as f64,
			height: CELL_SIZE * grid.num_rows () as f64,
		};

		let col_clues_size = Size {
			width: CELL_SIZE * grid.num_cols () as f64,
			height: CELL_SIZE * max_col_clues as f64,
		};

		let content_size = Size {

			width: (0.0
				+ BORDER_SIZE
				+ row_clues_size.width
				+ CLUE_GAP
				+ grid_size.width
				+ BORDER_SIZE
			),

			height: (0.0
				+ BORDER_SIZE
				+ grid_size.height
				+ CLUE_GAP
				+ col_clues_size.height
				+ BORDER_SIZE
			),

		};

		let row_clues_position = Position {
			horizontal: BORDER_SIZE,
			vertical: BORDER_SIZE + col_clues_size.height + CLUE_GAP + THICK_LINE_SIZE / 2.0,
		};

		let col_clues_position = Position {
			horizontal: BORDER_SIZE + row_clues_size.width + CLUE_GAP + THICK_LINE_SIZE / 2.0,
			vertical: BORDER_SIZE,
		};

		let grid_position = Position {
			horizontal: content_size.width - BORDER_SIZE - grid_size.width,
			vertical: content_size.height - BORDER_SIZE - grid_size.height,
		};

		SolverWindowDimensions {

			size: content_size,

			grid: Rectangle::from ( (
				grid_position,
				grid_size,
			) ),

			col_clues: Rectangle::from ( (
				col_clues_position,
				col_clues_size,
			) ),

			row_clues: Rectangle::from ( (
				row_clues_position,
				row_clues_size,
			) ),

		}

	}

	fn tick (& self) -> gtk::Continue {

		if self.solve_one_cell () {

			let state = self.state.borrow_mut ();
			let window = state.window.clone ().unwrap ();

			window.queue_draw ();

			gtk::Continue (true)

		} else {

			let mut state = self.state.borrow_mut ();

			state.timeout_source = None;

			gtk::Continue (false)

		}

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

		// content

		let content_width = state.dimensions.size.width;
		let content_height = state.dimensions.size.height;
		let content_ratio = content_width / content_height;

		let native_width = drawing_area.get_allocated_width () as f64;
		let native_height = drawing_area.get_allocated_height () as f64;
		let native_ratio = native_width / native_height;

		let scale = if native_ratio > content_ratio {
			native_height / content_height
		} else {
			native_width / content_width
		};

		context.translate (
			(native_width - content_width * scale) / 2.0,
			(native_height - content_height * scale) / 2.0,
		);

		context.scale (scale, scale);

		Self::draw_row_clues (& state, & context);
		Self::draw_col_clues (& state, & context);
		Self::draw_grid (& state, & context);

		// return

		gtk::Inhibit (false)

	}

	fn draw_row_clues (
		state: & SolverWindowState,
		context: & cairo::Context,
	) {

		let clues = state.solver.clues ();
		let palette = & state.palette;

		context.save ();

		context.translate (
			state.dimensions.row_clues.right,
			state.dimensions.row_clues.top,
		);

		let gtk_settings = gtk::Settings::get_default ().unwrap ();
		let gtk_font_name = gtk_settings.get_property_gtk_font_name ().unwrap ();

		let font_name = & gtk_font_name [
			0 .. gtk_font_name.chars ().rev ()
				.skip_while (|& ch| ch.is_ascii_digit ())
				.skip_while (|& ch| ch.is_whitespace ())
				.count ()
		];

		let font_face = context.select_font_face (
			& font_name,
			cairo::FontSlant::Normal,
			cairo::FontWeight::Normal,
		);

		context.set_font_size (CLUE_FONT_SIZE);

		for (row_index, row_clues) in clues.rows ().enumerate () {

			for (clue_index, clue) in row_clues.iter ().rev ().enumerate () {

				let text = format! ("{}", clue);
				let text_extents = context.text_extents (& text);

				let clue_position = Position {
					horizontal: - CELL_SIZE * clue_index as f64,
					vertical: CELL_SIZE * row_index as f64,
				};

				context.rectangle (
					clue_position.horizontal - CELL_SIZE,
					clue_position.vertical,
					CELL_SIZE,
					CELL_SIZE,
				);

				context.set_source (& state.palette.clue_box);
				context.fill ();

				context.move_to (
					clue_position.horizontal,
					clue_position.vertical,
				);

				context.rel_move_to (
					- (CELL_SIZE + text_extents.x_advance) / 2.0,
					(CELL_SIZE + text_extents.height) / 2.0,
				);

				context.set_source (& palette.clue_text);
				context.show_text (& text);

			}

		}

		context.restore ();

	}

	fn draw_col_clues (
		state: & SolverWindowState,
		context: & cairo::Context,
	) {

		let clues = state.solver.clues ();
		let palette = & state.palette;

		context.save ();

		context.translate (
			state.dimensions.col_clues.left,
			state.dimensions.col_clues.bottom,
		);

		let gtk_settings = gtk::Settings::get_default ().unwrap ();
		let gtk_font_name = gtk_settings.get_property_gtk_font_name ().unwrap ();

		let font_name = & gtk_font_name [
			0 .. gtk_font_name.chars ().rev ()
				.skip_while (|& ch| ch.is_ascii_digit ())
				.skip_while (|& ch| ch.is_whitespace ())
				.count ()
		];

		let font_face = context.select_font_face (
			& font_name,
			cairo::FontSlant::Normal,
			cairo::FontWeight::Normal,
		);

		context.set_font_size (CLUE_FONT_SIZE);

		for (col_index, col_clues) in clues.cols ().enumerate () {

			for (clue_index, clue) in col_clues.iter ().rev ().enumerate () {

				let text = format! ("{}", clue);
				let text_extents = context.text_extents (& text);

				let clue_position = Position {
					horizontal: CELL_SIZE * col_index as f64,
					vertical: - CELL_SIZE * clue_index as f64,
				};

				context.rectangle (
					clue_position.horizontal,
					clue_position.vertical - CELL_SIZE,
					CELL_SIZE,
					CELL_SIZE,
				);

				context.set_source (& state.palette.clue_box);
				context.fill ();

				context.move_to (
					clue_position.horizontal,
					clue_position.vertical,
				);

				context.rel_move_to (
					(CELL_SIZE - text_extents.x_advance) / 2.0,
					- (CELL_SIZE - text_extents.height) / 2.0,
				);

				context.set_source (& palette.clue_text);
				context.show_text (& text);

			}

		}

		context.restore ();

	}

	fn draw_grid (
		state: & SolverWindowState,
		context: & cairo::Context,
	) {

		let grid = state.solver.grid ();
		let palette = & state.palette;
		let grid_size = state.dimensions.grid.size ();

		let grid_size_internal = Size {
			width: grid_size.width - THICK_LINE_SIZE,
			height: grid_size.height - THICK_LINE_SIZE,
		};

		context.save ();

		context.translate (
			state.dimensions.grid.left + THICK_LINE_SIZE / 2.0,
			state.dimensions.grid.top + THICK_LINE_SIZE / 2.0,
		);

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
					CELL_SIZE * col_index as f64,
					CELL_SIZE * row_index as f64,
					CELL_SIZE + THIN_LINE_SIZE,
					CELL_SIZE + THIN_LINE_SIZE,
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

			context.move_to (0.0, CELL_SIZE * row_index as f64);
			context.rel_line_to (grid_size_internal.width, 0.0);
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

			context.move_to (CELL_SIZE * col_index as f64, 0.0);
			context.rel_line_to (0.0, grid_size_internal.height);
			context.stroke ();

		}

		context.restore ();

	}

	fn destroy (& self) {

		let mut state = self.state.borrow_mut ();

		if let Some (timeout_source) = state.timeout_source.take () {
			glib::source::source_remove (timeout_source);
		}

	}

}

#[ derive (Clone, Copy, Debug, Default) ]
struct Position {
	horizontal: f64,
	vertical: f64,
}

impl Position {

	fn origin () -> Position {
		Position {
			horizontal: 0.0,
			vertical: 0.0,
		}
	}

}

impl From <(f64, f64)> for Position {

	fn from (tuple: (f64, f64)) -> Position {
		Position {
			horizontal: tuple.0,
			vertical: tuple.1,
		}
	}

}

#[ derive (Clone, Copy, Debug, Default) ]
struct Size {
	width: f64,
	height: f64,
}

impl From <(f64, f64)> for Size {

	fn from (tuple: (f64, f64)) -> Size {
		Size {
			width: tuple.0,
			height: tuple.1,
		}
	}

}

#[ derive (Clone, Copy, Debug, Default) ]
struct Rectangle {
	left: f64,
	top: f64,
	right: f64,
	bottom: f64,
}

impl From <(Position, Size)> for Rectangle {

	fn from (tuple: (Position, Size)) -> Rectangle {
		Rectangle {
			left: tuple.0.horizontal,
			top: tuple.0.vertical,
			right: tuple.0.horizontal + tuple.1.width,
			bottom: tuple.0.vertical + tuple.1.height,
		}
	}

}

impl Rectangle {

	fn width (& self) -> f64 { self.right - self.left }
	fn height (& self) -> f64 { self.bottom - self.top }

	fn size (& self) -> Size {
		Size {
			width: self.width (),
			height: self.height (),
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
	clue_text: cairo::Pattern,
	clue_box: cairo::Pattern,
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
			clue_text:  Self::from_rgb (0.00, 0.00, 0.00),
			clue_box:   Self::from_rgb (0.85, 0.85, 0.85),
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

