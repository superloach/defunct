use std::rc::Rc;

use crate::runtime::block::Block;
use crate::runtime::calc::Calc;
use crate::runtime::error::Error;
use crate::runtime::lib::Lib;
use crate::runtime::native::{CallFn, Native};
use crate::runtime::queue::Queue;
use crate::runtime::text::Text;
use crate::runtime::under::Under;

#[derive(Debug, Clone)]
pub enum Funct {
	Under(Rc<Under>),
	Text(Rc<Text>),
	Block(Rc<Block>),
	Queue(Rc<Queue>),
	Calc(Rc<Calc>),
	Lib(Rc<Lib>),
	Error(Rc<Error>),
	Native(Rc<Native>),
	Zilch,
}

impl Funct {
	pub fn repr(self) -> Rc<Text> {
		match self {
			Self::Text(t) => t,
			Self::Under(_) => Text::new("_"),
			f => Text::new(format!("funct repr: stub {:?}", f)),
		}
	}

	pub fn call(self, under: Rc<Under>, args: Rc<Queue>) -> Self {
		match self {
			Self::Under(u) => u.call(args),
			Self::Text(t) => t.call(args),
			Self::Block(b) => b.call(under.child(args)),
			Self::Queue(q) => q.call(args),
			Self::Calc(c) => c.call(args),
			Self::Lib(l) => l.call(args),
			Self::Error(e) => e.call(args),
			Self::Native(n) => n.call(under, args),
			Self::Zilch => Self::Zilch,
		}
	}

	pub fn error<T: Into<Text>>(msg: T) -> Self {
		Self::Error(Error::new(msg))
	}

	pub fn text<T: Into<Text>>(text: T) -> Self {
		Self::Text(Text::new(text))
	}

	pub fn native(name: &'static str, call_fn: &'static CallFn) -> Self {
		Self::Native(Native::new(name, call_fn))
	}

	pub fn calc<T: Into<Calc>>(calc: T) -> Self {
		Self::Calc(Calc::new(calc))
	}
}
