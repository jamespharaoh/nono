use std::error::Error;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::num;
use std::path::Path;

use crate::data::*;

pub type CluesLine = Vec <LineSize>;

#[ derive (Debug) ]
pub struct Clues {
	rows: Vec <CluesLine>,
	cols: Vec <CluesLine>,
}

impl Clues {

	pub fn load_file (
		filename: & Path,
	) -> Result <Clues, Box <Error>> {

		let mut file = File::open (
			filename,
		) ?;

		Clues::load (& mut file)

	}

	pub fn load (
		reader: & mut io::Read,
	) -> Result <Clues, Box <Error>> {

		let reader = BufReader::new (
			reader,
		);

		let iter = reader.split (b'\n').map (
			|result| result.map (
				|line| String::from_utf8_lossy (& line).to_string (),
			),
		).map (
			|result| result.map (
				|line| line.trim ().to_string (),
			),
		).filter (
			|result| match result {
				Ok (line) => line != "",
				_ => true,
			}
		);

		#[ derive (PartialEq) ]
		enum Mode { None, Rows, Cols };
		let mut mode = Mode::None;

		let mut rows: Vec <Vec <LineSize>> = Vec::new ();
		let mut cols: Vec <Vec <LineSize>> = Vec::new ();

		for result in iter {

			let line = result ?;

			if line == "rows" {
				if mode != Mode::None {
					return Err ("parse error".into ());
				}
				mode = Mode::Rows;
				continue;
			}

			if line == "cols" {
				if mode != Mode::Rows {
					return Err ("parse error".into ());
				}
				mode = Mode::Cols;
				continue;
			}

			if mode == Mode::None {
				return Err ("parse error".into ());
			}

			let clues = line.split (" ").map (
				|text| text.parse::<LineSize> ()
			).collect::<Result <Vec <LineSize>, num::ParseIntError>> () ?;

			if mode == Mode::Rows {
				rows.push (clues);
			} else {
				cols.push (clues);
			}

		}

		Ok (Clues {
			rows: rows,
			cols: cols,
		})

	}

	pub fn num_rows (& self) -> LineSize {
		self.rows.len () as LineSize
	}

	pub fn num_cols (& self) -> LineSize {
		self.cols.len () as LineSize
	}

	pub fn rows (& self) -> impl Iterator <Item = & CluesLine> {
		self.rows.iter ()
	}

	pub fn cols (& self) -> impl Iterator <Item = & CluesLine> {
		self.cols.iter ()
	}

	pub fn row (& self, index: LineSize) -> & CluesLine {
		& self.rows [index as usize]
	}

	pub fn col (& self, index: LineSize) -> & CluesLine {
		& self.cols [index as usize]
	}

	pub fn rows_sum (& self) -> usize {
		self.rows.iter ().flatten ().map (|val| * val as usize).sum ()
	}

	pub fn cols_sum (& self) -> usize {
		self.cols.iter ().flatten ().map (|val| * val as usize).sum ()
	}

	pub fn is_consistent (& self) -> bool {
		self.rows_sum () == self.cols_sum ()
	}

}

