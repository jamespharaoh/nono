use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::num;
use std::path::Path;

use crate::line::LineSize;

pub type CluesLine = Vec <LineSize>;

pub struct Clues {
	pub rows: Vec <CluesLine>,
	pub cols: Vec <CluesLine>,
}

impl Clues {

	pub fn load (
		filename: & Path,
	) -> Result <Clues, Box <Error>> {

		let file = File::open (
			filename,
		) ?;

		let reader = BufReader::new (
			& file,
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
					panic! ("parse error");
				}
				mode = Mode::Rows;
				continue;
			}

			if line == "cols" {
				if mode != Mode::Rows {
					panic! ("parse error");
				}
				mode = Mode::Cols;
				continue;
			}

			if mode == Mode::None {
				panic! ("parse error");
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

}

