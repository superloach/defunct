use std::rc::Rc;

use crate::parser;
use crate::runtime::block::{Block, Expr, Value};
use crate::runtime::text::Text;

pub trait Interp<T> {
	fn interp(f: T) -> Rc<Self>;
}

impl Interp<parser::Block> for Block {
	fn interp(b: parser::Block) -> Rc<Self> {
		let mut exprs = Vec::new();
		for expr in b.0 {
			exprs.push(Expr::interp(expr));
		}
		Rc::new(Self(exprs))
	}
}

impl Interp<parser::Value> for Value {
	fn interp(v: parser::Value) -> Rc<Self> {
		match v {
			parser::Value::Under => Rc::new(Self::Under),
			parser::Value::Text(s) => Rc::new(Self::Text(Text::new(s))),
			parser::Value::Block(b) => Rc::new(Self::Block(Block::interp(b))),
		}
	}
}

impl Interp<parser::Expr> for Expr {
	fn interp(e: parser::Expr) -> Rc<Self> {
		let value = Value::interp(e.value);
		let mut calls = Vec::new();
		for call in e.calls.0 {
			calls.push(Block::interp(call));
		}
		Rc::new(Self { value, calls })
	}
}
