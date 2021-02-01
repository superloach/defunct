use crate::parser::chars;
use crate::parser::{Error, Parser};

impl Parser {
	pub fn parse_text(&mut self) -> Result<String, Error> {
		let mut s = String::new();
		loop {
			match self.peek() {
				Ok(chars::ESCAPE) => {
					self.index += 1;
					match self.peek() {
						Ok(c) => {
							self.index += 1;
							s.push(c);
						}
						Err(e) => return Err(e),
					}
				}
				Ok(c) => match c {
					chars::SPACE
					| chars::TAB
					| chars::NEWLINE
					| chars::UNDER
					| chars::BLOCK_OPEN
					| chars::BLOCK_CLOSE
					| chars::CALL_OPEN
					| chars::CALL_CLOSE => return Ok(s),
					c => {
						self.index += 1;
						s.push(c);
					}
				},
				Err(Error::NoMore) => return Ok(s),
				Err(e) => return Err(e),
			}
		}
	}
}
