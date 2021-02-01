use std::rc::Rc;

use crate::parser;
use crate::runtime::block::{Block, Expr, Value};
use crate::runtime::text::Text;

pub trait Interp<T> {
	fn interp(f: T) -> Rc<Self>;
}

impl Interp<parser::block::Block> for Block {
	fn interp(b: parser::block::Block) -> Rc<Self> {
		let mut exprs = Vec::new();
		for expr in b.0 {
			exprs.push(Expr::interp(expr));
		}
		Rc::new(Self(exprs))
	}
}

impl Interp<parser::value::Value> for Value {
	fn interp(v: parser::value::Value) -> Rc<Self> {
		match v {
			parser::value::Value::Under => Rc::new(Self::Under),
			parser::value::Value::Text(s) => Rc::new(Self::Text(Text::new(s))),
			parser::value::Value::Block(b) => Rc::new(Self::Block(Block::interp(b))),
		}
	}
}

impl Interp<parser::expr::Expr> for Expr {
	fn interp(e: parser::expr::Expr) -> Rc<Self> {
		let value = Value::interp(e.value);
		let mut calls = Vec::new();
		for call in e.calls {
			calls.push(Block::interp(call));
		}
		Rc::new(Self { value, calls })
	}
}
