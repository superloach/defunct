pub mod block;
pub mod chars;
pub mod expr;
pub mod text;
pub mod value;

#[derive(Debug)]
pub enum Error {
	NoMore,
	Unclosed,
}

pub struct Parser {
	pub index: usize,
	pub chars: Vec<char>,
}

impl Parser {
	pub fn new<T: AsRef<str>>(source: T) -> Self {
		let src = source.as_ref();
		Self {
			index: 0,
			chars: src.chars().collect(),
		}
	}

	pub fn peek(&self) -> Result<char, Error> {
		if self.index >= self.chars.len() {
			Err(Error::NoMore)
		} else {
			Ok(self.chars[self.index])
		}
	}

	pub fn skip_peek(&mut self) -> Result<char, Error> {
		loop {
			match self.peek() {
				Ok(chars::SPACE) | Ok(chars::TAB) | Ok(chars::NEWLINE) => self.index += 1,
				Ok(c) => return Ok(c),
				Err(e) => return Err(e),
			}
		}
	}
}
