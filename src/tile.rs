use std::fmt::{Display, Error, Formatter};

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Colour {
	Black,
	Red,
	Green,
	Brown,
	Blue,
	Magenta,
	Cyan,
	LightGrey,
	DarkGrey,
	LightRed ,
	LightGreen,
	Yellow,
	LightBlue,
	LightMagenta,
	LightCyan,
	White,
	Custom(u8),
	Transparent,
}

#[allow(dead_code)]
impl Colour {
	pub fn get_value(&self) -> u32 {
		match *self {
			Colour::Black => 0,
			Colour::Red => 1,
			Colour::Green => 2,
			Colour::Brown => 3,
			Colour::Blue => 4,
			Colour::Magenta => 5,
			Colour::Cyan => 6,
			Colour::LightGrey => 7,
			Colour::DarkGrey => 8,
			Colour::LightRed => 9,
			Colour::LightGreen => 10,
			Colour::Yellow => 11,
			Colour::LightBlue => 12,
			Colour::LightMagenta => 13,
			Colour::LightCyan => 14,
			Colour::White => 15,
			Colour::Custom(c) => c as u32,
			Colour::Transparent => 256,
		}
	}

	pub fn rgb_colour(mut r: u8, mut g: u8, mut b: u8) -> Colour {
		r = r / 51;
		g = g / 51;
		b = b / 51;

	    if r > 5 { r = 5; }
	    if g > 5 { g = 5; }
	 	if b > 5 { b = 5; }

	    Colour::Custom(16 + 36*r + 6*g + b)
	}

}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Tile {
	pub value: char,
	pub text_colour: Colour,
	pub background_colour: Colour
}

#[allow(dead_code)]
impl Tile {
	pub fn new() -> Tile {
		Tile {
			value: ' ',
			text_colour: Colour::Transparent,
			background_colour: Colour::Transparent
		}
	}

	pub fn new_text(value: char, text_colour: Colour) -> Tile {
		Tile {
			value: value,
			text_colour: text_colour,
			background_colour: Colour::Transparent
		}
	}

	pub fn new_background(background_colour: Colour) -> Tile {
		Tile {
			value: ' ',
			text_colour: Colour::Black,
			background_colour: background_colour
		}
	}

	pub fn new_text_background(value: char, text_colour: Colour, background_colour: Colour) -> Tile {
		Tile {
			value: value,
			text_colour: text_colour,
			background_colour: background_colour
		}
	}

	pub fn print_tile(&self) {
		if self.background_colour.get_value() != Colour::Transparent.get_value() {
			print!( "\x1B[48;5;{}m", self.background_colour.get_value() );
		}
		if self.text_colour.get_value() != Colour::Transparent.get_value() {
			print!( "\x1B[38;5;{}m", self.text_colour.get_value() );
		}
		print!("{}", self.value);

	}
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		let transparent = Colour::Transparent;

		let mut r = write!(f, "\x1B[48;5;{}m", self.background_colour.get_value());

		if self.background_colour.get_value() != Colour::Transparent.get_value() {
			r = write!(f, "\x1B[48;5;{}m", self.background_colour.get_value());
		}
		if self.text_colour.get_value() != transparent.get_value() {
			r = write!(f, "\x1B[38;5;{}m", self.text_colour.get_value());
		}
		r
    }
}
