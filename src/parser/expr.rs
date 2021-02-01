use crate::parser::block;
use crate::parser::chars;
use crate::parser::value;
use crate::parser::{Error, Parser};

#[derive(Debug)]
pub struct Expr {
	pub value: value::Value,
	pub calls: Vec<block::Block>,
}

impl Parser {
	pub fn parse_expr(&mut self) -> Result<Expr, Error> {
		match self.parse_value() {
			Ok(v) => {
				let mut cs = Vec::new();
				loop {
					match self.parse_wrap(chars::CALL_OPEN, chars::CALL_CLOSE) {
						Ok(b) => cs.push(b),
						Err(Error::NoMore) => {
							return Ok(Expr {
								value: v,
								calls: cs,
							})
						}
						Err(e) => return Err(e),
					}
				}
			}
			Err(e) => Err(e),
		}
	}
}
