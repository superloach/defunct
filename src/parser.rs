const SPACE: char = ' ';
const TAB: char = '\t';
const NEWLINE: char = '\n';
const UNDER: char = '_';
const BLOCK_OPEN: char = '{';
const BLOCK_CLOSE: char = '}';
const ESCAPE: char = '~';
const CALL_OPEN: char = '[';
const CALL_CLOSE: char = ']';

#[derive(Debug)]
pub struct Block(pub Vec<Expr>);

#[derive(Debug)]
pub struct Expr {
	pub value: Value,
	pub calls: Calls,
}

#[derive(Debug)]
pub enum Value {
	Under,
	Text(String),
	Block(Block),
}

#[derive(Debug)]
pub struct Calls(pub Vec<Block>);

#[derive(Debug)]
pub enum ParseErr {
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

	pub fn peek(&self) -> Result<char, ParseErr> {
		if self.index >= self.chars.len() {
			Err(ParseErr::NoMore)
		} else {
			Ok(self.chars[self.index])
		}
	}

	pub fn skip_peek(&mut self) -> Result<char, ParseErr> {
		loop {
			match self.peek() {
				Ok(SPACE) | Ok(TAB) | Ok(NEWLINE) => self.index += 1,
				Ok(c) => return Ok(c),
				Err(e) => return Err(e),
			}
		}
	}

	pub fn parse(&mut self) -> Result<Block, ParseErr> {
		let mut b = Block(Vec::new());
		loop {
			match self.parse_expr() {
				Ok(x) => b.0.push(x),
				Err(ParseErr::NoMore) => return Ok(b),
				Err(e) => return Err(e),
			}
		}
	}

	pub fn parse_expr(&mut self) -> Result<Expr, ParseErr> {
		match self.parse_value() {
			Ok(v) => {
				let mut cs = Calls(Vec::new());
				loop {
					match self.parse_wrap(CALL_OPEN, CALL_CLOSE) {
						Ok(b) => cs.0.push(b),
						Err(ParseErr::NoMore) => {
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

	pub fn parse_value(&mut self) -> Result<Value, ParseErr> {
		match self.skip_peek() {
			Ok(UNDER) => {
				self.index += 1;
				Ok(Value::Under)
			}
			Ok(c @ BLOCK_OPEN) => match self.parse_wrap(c, BLOCK_CLOSE) {
				Ok(w) => Ok(Value::Block(w)),
				Err(e) => Err(e),
			},
			Ok(_) => match self.parse_text() {
				Ok(s) if s.is_empty() => Err(ParseErr::NoMore),
				Ok(s) => Ok(Value::Text(s)),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}

	pub fn parse_wrap(&mut self, open: char, close: char) -> Result<Block, ParseErr> {
		match self.skip_peek() {
			Ok(c) if c == open => {
				self.index += 1;
				match self.parse() {
					Ok(s) => match self.skip_peek() {
						Ok(c) if c == close => {
							self.index += 1;
							Ok(s)
						}
						Err(ParseErr::NoMore) | Ok(_) => Err(ParseErr::Unclosed),
						Err(e) => Err(e),
					},
					Err(e) => Err(e),
				}
			}
			Ok(_) => Err(ParseErr::NoMore),
			Err(e) => Err(e),
		}
	}

	pub fn parse_text(&mut self) -> Result<String, ParseErr> {
		let mut s = String::new();
		loop {
			match self.peek() {
				Ok(ESCAPE) => {
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
					SPACE | TAB | NEWLINE | UNDER | BLOCK_OPEN | BLOCK_CLOSE | CALL_OPEN
					| CALL_CLOSE => return Ok(s),
					c => {
						self.index += 1;
						s.push(c);
					}
				},
				Err(ParseErr::NoMore) => return Ok(s),
				Err(e) => return Err(e),
			}
		}
	}
}
