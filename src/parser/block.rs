use crate::parser::expr;
use crate::parser::{Error, Parser};

#[derive(Debug)]
pub struct Block(pub Vec<expr::Expr>);

impl Parser {
	pub fn parse_block(&mut self) -> Result<Block, Error> {
		let mut b = Block(Vec::new());
		loop {
			match self.parse_expr() {
				Ok(x) => b.0.push(x),
				Err(Error::NoMore) => return Ok(b),
				Err(e) => return Err(e),
			}
		}
	}

	pub fn parse_wrap(&mut self, open: char, close: char) -> Result<Block, Error> {
		match self.skip_peek() {
			Ok(c) if c == open => {
				self.index += 1;
				match self.parse_block() {
					Ok(s) => match self.skip_peek() {
						Ok(c) if c == close => {
							self.index += 1;
							Ok(s)
						}
						Err(Error::NoMore) | Ok(_) => Err(Error::Unclosed),
						Err(e) => Err(e),
					},
					Err(e) => Err(e),
				}
			}
			Ok(_) => Err(Error::NoMore),
			Err(e) => Err(e),
		}
	}
}
