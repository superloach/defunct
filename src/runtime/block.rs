use std::rc::Rc;

use crate::runtime::funct::Funct;
use crate::runtime::queue::Queue;
use crate::runtime::text::Text;
use crate::runtime::under::Under;

#[derive(Debug)]
pub struct Block(pub Vec<Rc<Expr>>);

impl Block {
	pub fn eval(self: Rc<Self>, under: Rc<Under>) -> Rc<Queue> {
		let mut exprs = Vec::new();
		for expr in &self.0 {
			exprs.push(expr.clone().eval(under.clone()));
		}
		Queue::from(exprs)
	}

	pub fn call(self: Rc<Self>, under: Rc<Under>) -> Funct {
		let exprs = self.eval(under);
		{
			let bexprs = exprs.0.borrow();
			match bexprs.len() {
				0 => Funct::Zilch,
				l => bexprs[l - 1].clone(),
			}
		}
	}
}

#[derive(Debug)]
pub enum Value {
	Under,
	Text(Rc<Text>),
	Block(Rc<Block>),
}

impl Value {
	pub fn eval(self: Rc<Self>, under: Rc<Under>) -> Funct {
		match &*self {
			Self::Under => Funct::Under(under),
			Self::Text(t) => Funct::Text(t.clone()),
			Self::Block(b) => Funct::Block(b.clone()),
		}
	}
}

#[derive(Debug)]
pub struct Expr {
	pub value: Rc<Value>,
	pub calls: Vec<Rc<Block>>,
}

impl Expr {
	pub fn eval(self: Rc<Self>, under: Rc<Under>) -> Funct {
		let mut value = self.value.clone().eval(under.clone());
		for call in &self.calls {
			let args = call.clone().eval(under.clone());
			value = value.call(under.clone(), args.clone());
		}
		value
	}
}
