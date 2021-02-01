use crate::parser::block;
use crate::parser::chars;
use crate::parser::{Error, Parser};

#[derive(Debug)]
pub enum Value {
	Under,
	Text(String),
	Block(block::Block),
}

impl Parser {
	pub fn parse_value(&mut self) -> Result<Value, Error> {
		match self.skip_peek() {
			Ok(chars::UNDER) => {
				self.index += 1;
				Ok(Value::Under)
			}
			Ok(c @ chars::BLOCK_OPEN) => match self.parse_wrap(c, chars::BLOCK_CLOSE) {
				Ok(w) => Ok(Value::Block(w)),
				Err(e) => Err(e),
			},
			Ok(_) => match self.parse_text() {
				Ok(s) if s.is_empty() => Err(Error::NoMore),
				Ok(s) => Ok(Value::Text(s)),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}
}
